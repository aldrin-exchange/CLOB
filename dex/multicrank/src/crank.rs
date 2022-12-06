//! Initializing and spawning of subprocesses.

use std::{
    path::PathBuf,
    process::{Child, Command},
    time::Instant,
};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::{model::Market, state::Parameters};

/// A structure representing either a crank subprocess during runtime, or state
/// persistance on disk of a process that did not finish execution.
#[derive(Debug, Serialize, Deserialize)]
pub struct CrankInstance {
    /// Name of the market.
    pub name: Option<String>,
    /// Path to the crank binary.
    crank_bin: PathBuf,
    /// RPC endpoint.
    rpc: String,
    /// `--dex-program-id` parameter of the crank consume-events subcommand.
    pub dex_program_id: String,
    /// `--payer` parameter of the crank consume-events subcommand.
    payer: PathBuf,
    /// `--market` parameter of the crank consume-events subcommand.
    pub market: String,
    /// Mint of the base token.
    pub base_mint_address: Option<String>,
    /// Base token symbol.
    pub base_symbol: Option<String>,
    /// `--coin-wallet` parameter of the crank consume-events subcommand.
    pub coin_wallet: String,
    /// Mint of the quote token.
    pub quote_mint_address: Option<String>,
    /// Quote token symbol.
    pub quote_symbol: Option<String>,
    /// `--pc-wallet` parameter of the crank consume-events subcommand.
    pub pc_wallet: String,
    /// `--num-workers` parameter of the crank consume-events subcommand.
    num_workers: usize,
    /// `--events-per-worker` parameter of the crank consume-events subcommand.
    events_per_worker: usize,
    /// ` --log-directory` parameter of the crank consume-events subcommand.
    log_directory: PathBuf,
    /// Marks for how long should the child process run in minutes, `None`
    /// value representing forever.
    pub should_run_for: Option<u64>,
    /// Marks the moment when the child process has started. Does not persist
    /// across aborts and restarts, meaning it only marks the start of most
    /// recent execution.
    #[serde(skip)]
    pub start_time: Option<Instant>,
    /// Handle to the child process. Only useful during runtime, therefore not
    /// (can't be) persisted.
    #[serde(skip)]
    pub handle: Option<Child>,
}

impl CrankInstance {
    /// Construct an inactive market crank.
    pub fn new(
        params: &Parameters,
        market_info: Market,
        crank_duration: Option<u64>,
    ) -> Self {
        Self {
            name: market_info.name,
            crank_bin: params.crank.to_owned(),
            rpc: params.rpc.to_owned(),
            dex_program_id: market_info.program_id,
            payer: params.gas_payer.to_owned(),
            market: market_info.address.clone(),
            coin_wallet: market_info.base_token_account,
            pc_wallet: market_info.quote_token_account,
            num_workers: 1,
            events_per_worker: 10,
            log_directory: params
                .persist
                .join("logs")
                .join(&market_info.address),
            should_run_for: crank_duration,
            start_time: None,
            handle: None,
            base_mint_address: market_info.base_mint_address,
            base_symbol: market_info.base_symbol,
            quote_mint_address: market_info.quote_mint_address,
            quote_symbol: market_info.quote_symbol,
        }
    }

    /// Run the crank and update the structure. Fails on failed process
    /// spawn.
    pub fn start(&mut self) -> Result<()> {
        let args = self.arrange_args();
        tracing::debug!("starting `crank` with arguments: {args:?}");
        let child = Command::new(&self.crank_bin).args(args).spawn()?;
        self.start_time = Some(std::time::Instant::now());
        self.handle = Some(child);

        Ok(())
    }

    /// Stop the crank process.
    pub fn halt(&mut self) -> Result<()> {
        if let Some(ref mut handle) = self.handle {
            handle.kill()?;
            Ok(())
        } else {
            let market_id = &self.market;
            tracing::error!("something went wrong, no process handle for market {market_id}");
            Err(anyhow!("something went wrong, no process handle for market {market_id}"))
        }
    }

    /// Assemble `crank` arguments.
    fn arrange_args(&self) -> Vec<String> {
        vec![
            self.rpc.to_owned(),
            "consume-events".to_owned(),
            "--dex-program-id".to_owned(),
            self.dex_program_id.to_owned(),
            "--payer".to_owned(),
            self.payer.to_string_lossy().to_string(),
            "--market".to_owned(),
            self.market.to_owned(),
            "--coin-wallet".to_owned(),
            self.coin_wallet.to_owned(),
            "--pc-wallet".to_owned(),
            self.pc_wallet.to_owned(),
            "--num-workers".to_owned(),
            self.num_workers.to_string(),
            "--events-per-worker".to_owned(),
            self.events_per_worker.to_string(),
            "--log-directory".to_owned(),
            self.log_directory.to_string_lossy().to_string(),
        ]
    }
}
