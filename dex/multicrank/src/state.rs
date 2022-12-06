//! Global state and configuration implementation.

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use shellexpand::tilde;

use crate::{
    crank::CrankInstance,
    model::{Market, Markets},
    Arguments,
};

const DEFAULT_PERSIST_PATH: &str = "~/.multicrank";

/// Structure representing both state and configuration of the application,
/// either at runtime or persisted.
#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    /// Program arguments.
    pub params: Parameters,
    /// A map of running cranks where keys are market addresses.
    pub cranks: HashMap<String, CrankInstance>,
}

/// Similar to [`Arguments`], but with default values for missing fields.
#[derive(Serialize, Deserialize, Debug)]
pub struct Parameters {
    /// Path to the crank binary
    pub crank: PathBuf,
    /// RPC endpoint
    pub rpc: String,
    /// Path to gas payer id.json
    pub gas_payer: PathBuf,
    /// Server socket
    pub socket: u16,
    /// Path(s) to markets.json
    pub markets: PathBuf,
    /// Path to folder where state and logs will be stored
    pub persist: PathBuf,
}

impl From<&Arguments> for Parameters {
    fn from(args: &Arguments) -> Self {
        let markets_path = if let Some(ref path) = args.markets {
            path.to_owned()
        } else {
            PathBuf::from(&*tilde(DEFAULT_PERSIST_PATH)).join("markets.json")
        };
        let persist_path = if let Some(ref path) = args.persist {
            path.to_owned()
        } else {
            PathBuf::from(&*tilde(DEFAULT_PERSIST_PATH))
        };

        Self {
            crank: args.crank.to_owned(),
            rpc: args.rpc.to_owned(),
            gas_payer: args.gas_payer.to_owned(),
            socket: args.socket,
            markets: markets_path,
            persist: persist_path,
        }
    }
}

impl Parameters {
    fn canonicalize_paths(&mut self) -> Result<()> {
        self.crank = fs::canonicalize(&self.crank)?;
        self.gas_payer = fs::canonicalize(&self.gas_payer)?;
        self.markets = fs::canonicalize(&self.markets)?;

        Ok(())
    }
}

impl From<Parameters> for State {
    fn from(params: Parameters) -> Self {
        Self {
            params,
            cranks: HashMap::new(),
        }
    }
}

impl State {
    /// (Re)construct state from CLI arguments, either by loading old state, or
    /// by constructing new one.
    pub fn from_args(args: &Arguments) -> Result<Self> {
        match args.persist {
            Some(ref folder) => {
                let state_json_path = folder.join("state.json");

                if state_json_path.exists() {
                    tracing::debug!(
                        "restoring state from {state_json_path:?}..."
                    );
                    Ok(Self::from_json(&state_json_path)?)
                } else {
                    tracing::debug!("constructing state from args {args:?}...");
                    let mut state = Self::from_args_new(args)?;
                    state.import_cranks()?;
                    Ok(state)
                }
            }
            None => {
                let state_json_path =
                    PathBuf::from(&*tilde(DEFAULT_PERSIST_PATH))
                        .join("state.json");

                if state_json_path.exists() {
                    tracing::debug!(
                        "restoring state from {state_json_path:?}..."
                    );
                    Ok(Self::from_json(&state_json_path)?)
                } else {
                    tracing::debug!("constructing state from args {args:?}...");
                    let mut state = Self::from_args_new(args)?;
                    state.import_cranks()?;
                    Ok(state)
                }
            }
        }
    }

    /// Deserialize state from a persisted JSON.
    fn from_json(state_json: &Path) -> Result<Self> {
        let state_file = fs::read_to_string(state_json)?;
        let mut state_de: State = serde_json::from_str(&state_file)?;
        state_de.import_cranks()?;
        state_de.restart_cranks()?;

        Ok(state_de)
    }

    /// Construct new state from program arguments.
    fn from_args_new(args: &Arguments) -> Result<Self> {
        let mut params: Parameters = args.into();
        params.canonicalize_paths()?;

        Ok(params.into())
    }

    /// Add and start market crank instance.
    pub fn add_market(
        &mut self,
        market: Market,
        crank_duration: Option<u64>,
    ) -> Result<()> {
        let market_id = market.address.to_owned();
        let mut constructed_instance =
            CrankInstance::new(&self.params, market, crank_duration);
        constructed_instance.start()?;

        self.cranks.insert(market_id, constructed_instance);

        Ok(())
    }

    /// Remove market crank from state and return the removed market if there is
    /// one.
    pub fn purge_market(&mut self, market_id: &str) -> Result<Option<Market>> {
        if let Some(crank) = self.cranks.get_mut(market_id) {
            crank.halt()?;
            Ok(self.cranks.remove(market_id).map(|ref crank| crank.into()))
        } else {
            Err(anyhow!("something went wrong, cannot find crank with market id {market_id}"))
        }
    }

    /// Get a map of all active market cranks.
    pub fn get_cranks(&self) -> &HashMap<String, CrankInstance> {
        &self.cranks
    }

    /// Restart all the persisted cranks.
    fn restart_cranks(&mut self) -> Result<()> {
        for (market_id, crank) in self.cranks.iter_mut() {
            crank.start().map_err(|e| {
                anyhow!("could not restart crank for market {market_id}: {e}")
            })?;
        }

        Ok(())
    }

    /// Import and start market cranks from markets.json.
    fn import_cranks(&mut self) -> Result<()> {
        let markets_file = fs::read_to_string(&self.params.markets)?;
        let markets_de: Markets = serde_json::from_str(&markets_file)?;

        for (market_id, market) in markets_de.markets {
            if !self.cranks.contains_key(&market_id) {
                self.add_market(market, None)?;
            }
        }

        Ok(())
    }
}
