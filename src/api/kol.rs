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
    /// Every buy/sell from tracked KOLs' verified EVM wallets, attributed to the
    /// effective trading account (`tx.from`, or the ERC-4337 userOp sender when
    /// the trade was bundled), enriched with the token's MC/liquidity/peak, the
    /// deployer's reputation tier, and `mc_multiple_since_trade`.
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

    /// KOL clustering / consensus (`GET /rhc/kol/coordination`, BASIC+).
    ///
    /// Tokens bought by N+ DISTINCT tracked KOLs inside the window, ranked by
    /// KOL count then buy volume. Each row carries the per-KOL breakdown,
    /// `net_eth` (buys − sells in-window), a `signal` of `accumulating` vs
    /// `distributing`, `exited_count`/`holders_count`, and
    /// `time_to_consensus_sec` (how fast the cohort piled in).
    ///
    /// Deeper than [`hot_tokens`](Self::hot_tokens): that returns the ranked
    /// token list, this returns the cohort composition and exit state behind it.
    /// RHC has no KOL winrate/strategy materialized views, so the Solana
    /// `avg_winrate_7d` / `coordination_score` fields are intentionally absent.
    pub async fn coordination(&self, params: &CoordinationParams) -> Result<CoordinationResponse> {
        self.core.get("/rhc/kol/coordination", params).await
    }

    /// Earliest KOL entry per token (`GET /rhc/kol/first-touches`, BASIC+).
    ///
    /// The first time ANY tracked KOL bought a given token — the early-entry /
    /// discovery signal. Each event carries the entry size in ETH, `tx_hash`,
    /// `token_age_minutes` at first touch, the MC at entry and the current +
    /// peak MC, so you can score how the call aged.
    ///
    /// Tier depth: BASIC clamps `limit` to 20, and the KOL's `evm_address`
    /// inside `first_kol` is revealed only to ULTRA/BUSINESS (`name` and
    /// `twitter_url` are always returned).
    pub async fn first_touches(
        &self,
        params: &FirstTouchesParams,
    ) -> Result<FirstTouchesResponse> {
        self.core.get("/rhc/kol/first-touches", params).await
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
