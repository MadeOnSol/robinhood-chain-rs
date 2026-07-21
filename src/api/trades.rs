use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// The Robinhood Chain DEX trade tape.
#[derive(Debug, Clone)]
pub struct Trades {
    pub(crate) core: Arc<HttpCore>,
}

impl Trades {
    /// Robinhood Chain DEX trade tape (`GET /rhc/trades`, PRO+).
    ///
    /// Every Uniswap v2/v3/v4 swap on chain 4663 from our self-hosted node,
    /// ~sub-second from execution. Each row carries the real trader wallet
    /// (`trader_eoa`, tx.from — not the router), gas/ordering for MEV analysis,
    /// pool state, and KOL/deployer flags. Cursor via `next_before`.
    pub async fn list(&self, params: &TradesParams) -> Result<TradesResponse> {
        self.core.get("/rhc/trades", params).await
    }
}
