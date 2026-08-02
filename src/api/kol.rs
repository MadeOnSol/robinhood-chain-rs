use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// KOL trade-intelligence endpoints on Robinhood Chain — feed, leaderboard,
/// consensus hot-tokens, single-KOL profiles, and the two KOL push rule engines
/// (coordination alerts, first-touch subscriptions).
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

    // ── Rule engine: coordination alerts (PRO+) ──────────────────────────────

    /// Your coordination rules
    /// (`GET /rhc/kol/coordination/alerts`, PRO+).
    ///
    /// Rules that fire when N+ tracked KOLs buy the same token inside a rolling
    /// window — the push counterpart of the read-only
    /// [`coordination`](Self::coordination) endpoint.
    ///
    /// ⚠️ **Quotas are PER CHAIN**: a full set of Solana coordination rules does
    /// not consume any Robinhood Chain capacity.
    pub async fn coordination_alerts_list(&self) -> Result<CoordinationAlertListResponse> {
        self.core.get("/rhc/kol/coordination/alerts", &()).await
    }

    /// Create a coordination rule
    /// (`POST /rhc/kol/coordination/alerts`, PRO+).
    ///
    /// Scoring is the shared v1 scorer, so the number is comparable to Solana,
    /// but the `earliness` component is **defaulted** on RHC (there is no
    /// early-entry equivalent) while `quality` is a real KOL win-rate — the
    /// response's [`CoordinationAlertScoring`] block records which components
    /// are real, and every fired signal repeats it in `score_inputs`.
    ///
    /// The response carries `webhook_secret` **once**, minted whenever a
    /// `webhook_url` was supplied.
    pub async fn coordination_alerts_create(
        &self,
        params: &CoordinationAlertCreateParams,
    ) -> Result<CoordinationAlertCreateResponse> {
        self.core.post("/rhc/kol/coordination/alerts", params).await
    }

    /// Fetch one coordination rule
    /// (`GET /rhc/kol/coordination/alerts/{id}`, PRO+).
    ///
    /// `id` is the rule **UUID**. Returns a 404 [`Error::Api`](crate::Error)
    /// when you do not own it.
    pub async fn coordination_alerts_get(
        &self,
        id: &str,
    ) -> Result<CoordinationAlertGetResponse> {
        self.core
            .get(&format!("/rhc/kol/coordination/alerts/{}", id), &())
            .await
    }

    /// Partially update a coordination rule
    /// (`PATCH /rhc/kol/coordination/alerts/{id}`, PRO+).
    ///
    /// Omitted fields are left untouched. Send `min_mc_usd` and `max_mc_usd`
    /// together when changing the band — the ordering check only runs when both
    /// are present in the same body.
    pub async fn coordination_alerts_update(
        &self,
        id: &str,
        params: &CoordinationAlertUpdateParams,
    ) -> Result<CoordinationAlertGetResponse> {
        self.core
            .patch(&format!("/rhc/kol/coordination/alerts/{}", id), params)
            .await
    }

    /// Delete a coordination rule
    /// (`DELETE /rhc/kol/coordination/alerts/{id}`, PRO+).
    ///
    /// Its cooldown state and fired signals cascade.
    pub async fn coordination_alerts_delete(&self, id: &str) -> Result<RhcDeletedResponse> {
        self.core
            .delete(&format!("/rhc/kol/coordination/alerts/{}", id))
            .await
    }

    // ── Rule engine: first-touch subscriptions (ULTRA+) ──────────────────────

    /// Your first-touch subscriptions
    /// (`GET /rhc/kol/first-touches/subscriptions`, **ULTRA+**).
    ///
    /// Push subscriptions that fire when a token gets its FIRST tracked-KOL
    /// buy — the push counterpart of the read-only
    /// [`first_touches`](Self::first_touches) endpoint.
    ///
    /// ⚠️ **Quotas are PER CHAIN.** Note this family is ULTRA+, one tier above
    /// the other three RHC rule engines.
    pub async fn first_touch_subscriptions_list(
        &self,
    ) -> Result<FirstTouchSubscriptionListResponse> {
        self.core
            .get("/rhc/kol/first-touches/subscriptions", &())
            .await
    }

    /// Create a first-touch subscription
    /// (`POST /rhc/kol/first-touches/subscriptions`, **ULTRA+**).
    ///
    /// The filter set is deliberately not the Solana one: RHC has no scout
    /// score, so `min_scout_tier` / `min_n_touches` are **absent** rather than
    /// silently matching nothing — [`FirstTouchFilters::min_kol_winrate`] and
    /// [`FirstTouchFilters::strategy`] are the quality gates. Unknown filter
    /// keys are rejected with a 400, not ignored.
    ///
    /// The response carries `webhook_secret` **once**, minted whenever a
    /// `webhook_url` was supplied.
    pub async fn first_touch_subscriptions_create(
        &self,
        params: &FirstTouchSubscriptionCreateParams,
    ) -> Result<FirstTouchSubscriptionCreateResponse> {
        self.core
            .post("/rhc/kol/first-touches/subscriptions", params)
            .await
    }

    /// Fetch one first-touch subscription
    /// (`GET /rhc/kol/first-touches/subscriptions/{id}`, **ULTRA+**).
    ///
    /// `id` is the subscription **UUID**.
    pub async fn first_touch_subscriptions_get(
        &self,
        id: &str,
    ) -> Result<FirstTouchSubscriptionGetResponse> {
        self.core
            .get(&format!("/rhc/kol/first-touches/subscriptions/{}", id), &())
            .await
    }

    /// Update a first-touch subscription
    /// (`PATCH /rhc/kol/first-touches/subscriptions/{id}`, **ULTRA+**).
    ///
    /// `filters` is a whole-object **replace**, not a merge — merging would make
    /// "remove this filter" inexpressible.
    pub async fn first_touch_subscriptions_update(
        &self,
        id: &str,
        params: &FirstTouchSubscriptionUpdateParams,
    ) -> Result<FirstTouchSubscriptionGetResponse> {
        self.core
            .patch(
                &format!("/rhc/kol/first-touches/subscriptions/{}", id),
                params,
            )
            .await
    }

    /// Delete a first-touch subscription
    /// (`DELETE /rhc/kol/first-touches/subscriptions/{id}`, **ULTRA+**).
    pub async fn first_touch_subscriptions_delete(
        &self,
        id: &str,
    ) -> Result<RhcDeletedResponse> {
        self.core
            .delete(&format!("/rhc/kol/first-touches/subscriptions/{}", id))
            .await
    }
}
