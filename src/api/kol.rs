use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// KOL trade-intelligence endpoints on Robinhood Chain — feed, leaderboard,
/// consensus hot-tokens, and single-KOL profiles.
///
/// The KOL→EVM mapping is recovered by tracing each Solana KOL's Solana→EVM
/// bridge deposits (deBridge / Relay / Mayan / Wormhole) — a dataset unique to
/// MadeOnSol.
#[derive(Debug, Clone)]
pub struct Kol {
    pub(crate) core: Arc<HttpCore>,
}

impl Kol {
    /// Real-time KOL trade feed on Robinhood Chain (`GET /rhc/kol/feed`, BASIC+).
    ///
    /// Every buy/sell from tracked KOLs' verified EVM wallets, attributed via
    /// `tx.from`, enriched with the token's MC/liquidity/peak, the deployer's
    /// reputation tier, and `mc_multiple_since_trade`.
    pub async fn feed(&self, params: &KolFeedParams) -> Result<KolFeedResponse> {
        self.core.get("/rhc/kol/feed", params).await
    }

    /// KOL activity leaderboard (`GET /rhc/kol/leaderboard`, BASIC+).
    ///
    /// KOLs ranked by trade count, then net ETH flow, over the chosen window.
    pub async fn leaderboard(
        &self,
        params: &KolLeaderboardParams,
    ) -> Result<KolLeaderboardResponse> {
        self.core.get("/rhc/kol/leaderboard", params).await
    }

    /// Consensus hot-tokens — bought by 2+ KOLs (`GET /rhc/kol/hot-tokens`, BASIC+).
    pub async fn hot_tokens(&self, params: &HotTokensParams) -> Result<HotTokensResponse> {
        self.core.get("/rhc/kol/hot-tokens", params).await
    }

    /// Single KOL profile (`GET /rhc/kol/{wallet}`, BASIC+).
    ///
    /// Aggregate stats over the KOL's last 200 RHC trades plus their 50 most
    /// recent trades. `wallet` is an EVM address (0x, 40 hex). Returns a 404
    /// [`Error::Api`](crate::Error) when the address has no RHC activity.
    pub async fn wallet(&self, wallet: &str) -> Result<KolProfileResponse> {
        self.core.get(&format!("/rhc/kol/{}", wallet), &()).await
    }
}
