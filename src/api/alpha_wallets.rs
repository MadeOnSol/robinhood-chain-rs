use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Smart-money wallet ranking on Robinhood Chain.
#[derive(Debug, Clone)]
pub struct AlphaWallets {
    pub(crate) core: Arc<HttpCore>,
}

impl AlphaWallets {
    /// Smart-money wallet ranking (`GET /rhc/alpha-wallets`, PRO+).
    ///
    /// Robinhood Chain trader wallets ranked by realized on-chain performance —
    /// the reverse of KOL discovery. `net_eth` is realized net flow (sell − buy);
    /// `win_rate` is the share of traded tokens taken out profitably; `likely_bot`
    /// flags atomic-arb/MM fleets. Because RHC is dual-natured (launchpad
    /// memecoins vs tokenized stocks/stables), filter with `min_memecoin_share`
    /// to isolate memecoin traders, or `max_avg_mc_usd` for low-caps. Page with
    /// `limit`/`offset` until `has_more` is false.
    pub async fn list(&self, params: &AlphaWalletsParams) -> Result<AlphaWalletsResponse> {
        self.core.get("/rhc/alpha-wallets", params).await
    }
}
