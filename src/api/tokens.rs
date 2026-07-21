use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Token intelligence on Robinhood Chain — discovery, per-token snapshot,
/// OHLC candles, KOL consensus, buyer-quality scoring, and launch-bundle detection.
#[derive(Debug, Clone)]
pub struct Tokens {
    pub(crate) core: Arc<HttpCore>,
}

impl Tokens {
    /// Token discovery (`GET /rhc/tokens`, PRO+).
    ///
    /// Live-priced Robinhood Chain tokens with market cap, liquidity, peak MC +
    /// drawdown, launchpad, and deployer reputation tier. Sortable and filterable.
    pub async fn list(&self, params: &TokensListParams) -> Result<TokensListResponse> {
        self.core.get("/rhc/tokens", params).await
    }

    /// Single-token bundle snapshot (`GET /rhc/tokens/{address}`, BASIC+).
    ///
    /// Full snapshot for one token: metadata, live price/MC/FDV, peak MC +
    /// drawdown, graduation status, deployer reputation block (+ other tokens by
    /// the same deployer), KOL activity summary, and pool inventory. `address`
    /// is an EVM token address (0x, 40 hex). Returns 404 when the token is not
    /// found on Robinhood Chain.
    pub async fn get(&self, address: &str) -> Result<TokenDetailResponse> {
        self.core
            .get(&format!("/rhc/tokens/{}", address), &())
            .await
    }

    /// 1-minute OHLC candles (`GET /rhc/tokens/{address}/candles`, PRO+).
    ///
    /// Price + market-cap OHLC, close liquidity, volume with buy/sell split, and
    /// trade/buy/sell counts, ordered oldest→newest. Use [`CandlesParams`] to
    /// pick `limit` (1..=1000) and an optional `from`/`to` window.
    pub async fn candles(
        &self,
        address: &str,
        params: &CandlesParams,
    ) -> Result<CandlesResponse> {
        self.core
            .get(&format!("/rhc/tokens/{}/candles", address), params)
            .await
    }

    /// KOL consensus on a token (`GET /rhc/tokens/{address}/kol-consensus`, PRO+).
    ///
    /// Distinct KOL buyers vs sellers, exit rate, `net_flow_eth`, median entry
    /// MC, and first-touch wallet/time. `consensus` is `None` when no tracked
    /// KOL has traded the token. ULTRA additionally returns the `buyers` and
    /// `exited` wallet lists.
    pub async fn kol_consensus(&self, address: &str) -> Result<KolConsensusResponse> {
        self.core
            .get(&format!("/rhc/tokens/{}/kol-consensus", address), &())
            .await
    }

    /// Early-buyer quality (`GET /rhc/tokens/{address}/buyer-quality`, BASIC+).
    ///
    /// A 0–100 quality read on a token's earliest distinct buyer cohort (first
    /// 20): win-rate, KOL-presence, bot-domination, bundle-buyer legs, plus the
    /// informational dump-cluster ensemble. Neutral score (50) with a `note`
    /// when the token has no buyer history yet.
    pub async fn buyer_quality(&self, address: &str) -> Result<BuyerQualityResponse> {
        self.core
            .get(&format!("/rhc/tokens/{}/buyer-quality", address), &())
            .await
    }

    /// Launch-bundle detection (`GET /rhc/tokens/{address}/bundle`, BASIC+).
    ///
    /// Ranks the first 20 distinct buyers by on-chain order, flags a bundle when
    /// 3+ of them make their first buy in the same block (`bundle_kind =
    /// same_block`, else `none` — there is no `atomic_tx` on this L2), and
    /// reports how much of what the cohort bought it still holds. Field-gated by
    /// tier: BASIC gets the scalar `bundle`; PRO adds the top-10 wallets; ULTRA
    /// returns the full cohort with alpha-wallet identity.
    pub async fn bundle(&self, address: &str) -> Result<RhcBundleResponse> {
        self.core
            .get(&format!("/rhc/tokens/{}/bundle", address), &())
            .await
    }
}
