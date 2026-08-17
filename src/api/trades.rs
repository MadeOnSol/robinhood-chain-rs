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
    /// ~sub-second from execution. Each row carries the effective trading
    /// account (`trader_eoa` — tx.from, or the ERC-4337 userOp sender when the
    /// trade was bundled; never the router or the bundler), gas/ordering for MEV
    /// analysis, pool state, and KOL/deployer flags. Cursor via `next_before`.
    pub async fn list(&self, params: &TradesParams) -> Result<TradesResponse> {
        self.core.get("/rhc/trades", params).await
    }

    /// Liquidity REMOVALS feed — the rug signal (`GET /rhc/lp-events`, PRO+).
    ///
    /// Every Uniswap v2/v3 `Burn` and every v4 `ModifyLiquidity` with a
    /// negative delta on tracked pools, decoded from our own node's log
    /// subscription, newest first. **Removals only** — liquidity adds are not
    /// persisted (v4 adds share the topic and are dropped at decode time; v2/v3
    /// `Mint` is not subscribed), so every row is `event: "remove"`, an empty
    /// page means "no removals seen" (never "no liquidity activity"), and
    /// [`LpEventsResponse::coverage`] says `adds_persisted: false`.
    ///
    /// Amounts (`liquidity`, `amount0`, `amount1`, `token_amount_raw`,
    /// `quote_amount_raw`) are raw uint256 integers as decimal **strings**; v4
    /// rows carry `liquidity` only (the pool manager emits no token amounts).
    /// `provider_is_token_deployer` is the classic rug tell. Cursor via
    /// `next_before` (same opaque `(block_time, id)` keyset as [`Self::list`]).
    /// Data since 2026-08-05.
    pub async fn lp_events(&self, params: &LpEventsParams) -> Result<LpEventsResponse> {
        self.core.get("/rhc/lp-events", params).await
    }
}
