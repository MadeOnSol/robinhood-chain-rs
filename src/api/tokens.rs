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

    /// Up to **50** tokens in ONE call (`POST /rhc/token/batch`, BASIC+).
    ///
    /// Set-based — three queries server-side regardless of batch size, not a
    /// fan-out of [`get`](Self::get). Each entry returns metadata, live
    /// price/MC/FDV/liquidity, peak MC, primary DEX and the deployer reputation
    /// block. Every REQUESTED address is echoed back, unknown ones as
    /// `found: false`, so positions line up with what you sent.
    ///
    /// Narrower than [`get`](Self::get) on purpose: it does NOT bundle
    /// buyer-quality (a per-token cohort computation) — use
    /// [`batch_buyer_quality`](Self::batch_buyer_quality) for that.
    ///
    /// Returns a 400 [`Error::Api`](crate::Error) when the list is empty, over
    /// 50, or contains a non-EVM address.
    ///
    /// ```no_run
    /// # async fn run(client: robinhood_chain::RobinhoodChain) -> Result<(), Box<dyn std::error::Error>> {
    /// let batch = client.tokens.batch(&["0xabc…".to_string(), "0xdef…".to_string()]).await?;
    /// println!("{}/{} found", batch.found, batch.requested);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn batch(&self, addresses: &[String]) -> Result<TokenBatchResponse> {
        let body = BatchAddressesRequest {
            addresses: addresses.to_vec(),
        };
        self.core.post("/rhc/token/batch", &body).await
    }

    /// Early-buyer quality for up to **20** tokens
    /// (`POST /rhc/tokens/batch/buyer-quality`, BASIC+).
    ///
    /// Batched [`buyer_quality`](Self::buyer_quality): the 0–100 read on each
    /// token's earliest distinct buyer cohort. Per-token failures degrade to an
    /// entry carrying `error` rather than failing the whole batch, so one
    /// unpriced token never costs you the other 19 results.
    ///
    /// ⚠️ The cap is **20**, deliberately lower than the Solana batch cap of 50:
    /// RHC buyer-quality is a per-token cohort computation (ordered early-buyer
    /// scan + bundle detection + alpha/cluster joins), not one set-based query,
    /// so 50 would mean ~200 round-trips behind a single request. The cap is
    /// echoed back as `max_addresses`, including on the 400.
    pub async fn batch_buyer_quality(
        &self,
        addresses: &[String],
    ) -> Result<BatchBuyerQualityResponse> {
        let body = BatchAddressesRequest {
            addresses: addresses.to_vec(),
        };
        self.core
            .post("/rhc/tokens/batch/buyer-quality", &body)
            .await
    }
}
