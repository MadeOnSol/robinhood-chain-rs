use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Deployer reputation on Robinhood Chain.
///
/// Most RHC launchpads are direct-to-DEX (no bonding curve), so "graduation" is
/// a market-cap milestone: `graduation_rate` = share of a deployer's tokens that
/// reached a $40K+ peak MC; `runner_rate` = share that reached $100K+.
///
/// ⚠️ **Tier semantics** (migrations 267 + 269): `elite` / `good` are earned on
/// the $100K `runner_rate` and require 24h of deployer history — the $40K bar
/// proved farmable by operators mass-relaunching one ticker across rotating
/// wallets. `graduation_rate` is still returned and still means the $40K bar,
/// but it NO LONGER determines the tier; `spammer` is the one exception that
/// still keys off it. [`stats`](Self::stats) returns the live thresholds.
#[derive(Debug, Clone)]
pub struct DeployerHunter {
    pub(crate) core: Arc<HttpCore>,
}

impl DeployerHunter {
    /// Deployer reputation leaderboard (`GET /rhc/deployer-hunter/leaderboard`, BASIC+).
    ///
    /// Deployers ranked by reputation, from a 5-min-refresh rollup over every
    /// launchpad token we've indexed (40k+ deployers). Page with `limit`/`offset`
    /// until `has_more` is false.
    pub async fn leaderboard(
        &self,
        params: &DeployerLeaderboardParams,
    ) -> Result<DeployerLeaderboardResponse> {
        self.core
            .get("/rhc/deployer-hunter/leaderboard", params)
            .await
    }

    /// Single deployer profile (`GET /rhc/deployer-hunter/{address}`, BASIC+).
    ///
    /// One deployer's full reputation row plus their 50 most recent tokens
    /// enriched with live MC and peak MC. Unknown wallets return 200 with
    /// `is_deployer: false` (not a 404) so clients can branch cheaply. `address`
    /// is an EVM wallet address (0x, 40 hex).
    pub async fn profile(&self, address: &str) -> Result<DeployerProfileResponse> {
        self.core
            .get(&format!("/rhc/deployer-hunter/{}", address), &())
            .await
    }

    /// Is this deployer getting better or worse?
    /// (`GET /rhc/deployer-hunter/{address}/trajectory`, BASIC+).
    ///
    /// A shape-over-time read on one deployer's launch history: current and
    /// longest hit/miss streaks, a rolling 10-launch success rate, the best and
    /// worst 10-launch stretches, average days between deploys, average launches
    /// burned between a miss and the next hit, and a `trend`.
    ///
    /// The per-token success event is the **$40K peak-MC graduation** milestone
    /// (echoed as `success_metric`), not the $100K runner bar that TIERS ride
    /// on — $100K is rare enough that most deployers would return an all-zero
    /// curve, and a trajectory needs enough events to have a shape. Field names
    /// keep the Solana `bond` wording so the two chains stay drop-in compatible.
    ///
    /// Capped at the 500 oldest→newest launches; `truncated` tells you whether
    /// the curve is the whole story. Unknown wallets return 200 with
    /// `is_deployer: false` (not a 404).
    pub async fn trajectory(&self, address: &str) -> Result<DeployerTrajectoryResponse> {
        self.core
            .get(&format!("/rhc/deployer-hunter/{}/trajectory", address), &())
            .await
    }

    /// A deployer's full paginated launch history
    /// (`GET /rhc/deployer-hunter/{address}/tokens`, BASIC+).
    ///
    /// Every token this deployer shipped, enriched with live MC, peak MC and
    /// liquidity. Distinct from [`profile`](Self::profile), which caps
    /// `recent_tokens` at 50 and is a point-in-time read — this is the
    /// enumerable history with `limit`/`offset` and `has_more`.
    ///
    /// ⚠️ [`DeployerTokensSort::PeakMcUsd`] is a **page-scoped** sort (the
    /// response echoes `sort_scope: "page"`); use [`best_tokens`](Self::best_tokens)
    /// for a real ranking. Unknown wallets return 200 with `is_deployer: false`.
    pub async fn tokens(
        &self,
        address: &str,
        params: &DeployerTokensParams,
    ) -> Result<DeployerTokensResponse> {
        self.core
            .get(&format!("/rhc/deployer-hunter/{}/tokens", address), params)
            .await
    }

    /// Best tokens from reputable deployers
    /// (`GET /rhc/deployer-hunter/best-tokens`, BASIC+).
    ///
    /// The highest-peaking tokens launched in the window by deployers that are
    /// currently `elite` or `good` — "what did the deployers worth tracking
    /// actually produce". Gated on reputation, not raw peak MC; the unfiltered
    /// version is [`Tokens::list`](crate::api::tokens::Tokens::list) with
    /// [`TokensSort::PeakMc`].
    ///
    /// Reputation here means the $100K `runner_rate` tier, not the $40K
    /// `graduation_rate` — the latter is still on each token's `deployer` block
    /// but no longer sets the tier. `candidates_scanned` reports how many
    /// launches were considered; when `truncated` is true the top-N was drawn
    /// from the 1000 most RECENT launches rather than the whole period.
    pub async fn best_tokens(&self, params: &BestTokensParams) -> Result<BestTokensResponse> {
        self.core
            .get("/rhc/deployer-hunter/best-tokens", params)
            .await
    }

    /// Chain-wide deployer reputation summary
    /// (`GET /rhc/deployer-hunter/stats`, BASIC+).
    ///
    /// Population per tier (deployers + tokens), the reputable-deployer count,
    /// `spam_token_share`, and 24h/7d alert volume — the denominator for any
    /// "is this deployer rare?" question.
    ///
    /// Also returns `tier_rules`, the ACTIVE thresholds, so you can see what
    /// `elite` currently means instead of guessing from the label:
    /// `elite`/`good` are earned on the $100K `runner_rate` and require 24h of
    /// deployer history, while `spammer` still keys off `graduation_rate`.
    pub async fn stats(&self) -> Result<DeployerStatsResponse> {
        self.core.get("/rhc/deployer-hunter/stats", &()).await
    }

    /// Deployer signal feed (`GET /rhc/deployer-hunter/alerts`, BASIC+).
    ///
    /// Live alerts when a tracked deployer ships a new token or one of their
    /// tokens graduates ($40K+ peak MC). RHC domain values are narrower than
    /// Solana's: `alert_type` is `new_deploy`/`graduated` (no bonded/kol_buy),
    /// `priority` is `high`/`medium` (no low), and alerts carry no KOL join.
    ///
    /// ⚠️ **The tradability filter is ON by default**: alerts whose token has
    /// `liquidity_usd` below $100 — or unknown liquidity, which on RHC usually
    /// means a drained pool — are dropped, because a $45K-MC alert on a token
    /// with $68 of liquidity is not a signal. Set
    /// `include_untradeable: Some(true)` for the raw tape (archive/leaderboard
    /// tooling). The active setting is echoed as `tradability_filter`.
    ///
    /// ⚠️ **`tier` is resolved at read time**: it is the deployer's CURRENT tier
    /// from the reputation view, not the snapshot taken when the alert fired —
    /// that snapshot comes back as `tier_at_alert`, with `tier_is_stale` set when
    /// the two disagree. The `deployer_tier` filter matches the RESOLVED value so
    /// the filter and the payload always agree.
    pub async fn alerts(&self, params: &DeployerAlertsParams) -> Result<DeployerAlertsResponse> {
        self.core.get("/rhc/deployer-hunter/alerts", params).await
    }

    /// A deployer's deploy history with graduation detail
    /// (`GET /rhc/deployer-hunter/{address}/history`, PRO+).
    ///
    /// The reputation row plus every token deployed — newest first, with a
    /// stable tiebreaker so paginated pages never overlap or skip — enriched
    /// with live and peak MC and the `graduated_pool`. `total` is an exact count.
    ///
    /// RHC has no per-day reputation snapshot table (that is Solana-only), so
    /// unlike the Solana history route this is a token-deploy history, not a daily
    /// tier/rate time-series; for the shape-over-time read use
    /// [`trajectory`](Self::trajectory). Unknown wallets return 200 with
    /// `is_deployer: false`.
    pub async fn history(
        &self,
        address: &str,
        params: &DeployerHistoryParams,
    ) -> Result<DeployerHistoryResponse> {
        self.core
            .get(&format!("/rhc/deployer-hunter/{}/history", address), params)
            .await
    }

    /// Recent graduations (`GET /rhc/deployer-hunter/recent-bonds`, BASIC+).
    ///
    /// Tokens that crossed the $40K peak-MC graduation milestone, newest peak
    /// first, with token metadata and the deployer's tier. On RHC a "graduation"
    /// is that MC milestone, NOT a bonding-curve completion — noxa/pons/clanker
    /// launch direct-to-DEX with no curve — so the set is defined purely by
    /// `peak_mc_usd >= 40000`.
    pub async fn recent_bonds(&self, params: &RecentBondsParams) -> Result<RecentBondsResponse> {
        self.core
            .get("/rhc/deployer-hunter/recent-bonds", params)
            .await
    }
}
