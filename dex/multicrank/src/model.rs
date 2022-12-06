//! Various data models.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::crank::CrankInstance;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub address: String,
    pub deprecated: Option<bool>,
    pub name: Option<String>,
    pub program_id: String,
    pub base_mint_address: Option<String>,
    pub quote_mint_address: Option<String>,
    pub base_token_account: String,
    pub quote_token_account: String,
    pub base_symbol: Option<String>,
    pub quote_symbol: Option<String>,
}

impl From<&CrankInstance> for Market {
    fn from(crank: &CrankInstance) -> Self {
        Self {
            address: crank.market.to_owned(),
            deprecated: Some(false),
            name: crank.name.to_owned(),
            program_id: crank.dex_program_id.to_owned(),
            base_mint_address: crank.base_mint_address.to_owned(),
            quote_mint_address: crank.quote_mint_address.to_owned(),
            base_token_account: crank.coin_wallet.to_owned(),
            quote_token_account: crank.pc_wallet.to_owned(),
            base_symbol: crank.base_symbol.to_owned(),
            quote_symbol: crank.quote_symbol.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Markets {
    #[serde(flatten)]
    pub markets: HashMap<String, Market>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrankRequest {
    pub market_info: Market,
    pub crank_duration: Option<u64>,
}
