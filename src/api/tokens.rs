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

    /// Tokenized stocks & ETFs (`GET /rhc/equities`, BASIC+).
    ///
    /// Every official Robinhood tokenized equity (NVDA, SPY, AAPL, …) with live
    /// price / MC / liquidity and 24h trades, ETH volume, buys/sells and distinct
    /// buyers/sellers. **Identity is the issuer beacon, never the name**: a token
    /// is listed only if its contract is an EIP-1967 beacon proxy on Robinhood's
    /// issuer beacon `0xe10b6f6b275de231345c20d14ab812db62151b00`, read from our
    /// own node (re-classified every 10 min) — look-alike "GameStop • Robinhood
    /// Token" contracts are excluded by construction and `issuer_beacon` is
    /// echoed per row. Sort with [`EquitiesSort`] (default `volume`); `symbol`
    /// is an exact case-insensitive ticker, `q` a substring of symbol/name.
    /// 24h stats are cached 60 s (`stats_as_of`).
    pub async fn equities(&self, params: &EquitiesParams) -> Result<EquitiesResponse> {
        self.core.get("/rhc/equities", params).await
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

    /// Top traders of one token (`GET /rhc/tokens/{address}/top-traders`, PRO+).
    ///
    /// Lifetime per-trader performance, ranked, enriched with wallet reputation
    /// (win-rate, bot heuristic, KOL identity), dump-cluster membership and
    /// early-buyer rank.
    ///
    /// **`net_eth` is REALIZED flow (`sell − buy`), not PnL.** It does not value a
    /// trader's remaining bag, so a wallet that bought and still holds ranks
    /// **last**, not first. Use the wallet PnL endpoint for FIFO cost-basis PnL.
    ///
    /// 50 rows on PRO; ULTRA/BUSINESS raises the cap to 200.
    pub async fn top_traders(
        &self,
        address: &str,
        params: &TopTradersParams,
    ) -> Result<serde_json::Value> {
        self.core
            .get(&format!("/rhc/tokens/{}/top-traders", address), params)
            .await
    }

    /// Net buy/sell flow by trader cohort (`GET /rhc/tokens/{address}/flow`, PRO+).
    ///
    /// **Sign convention: `net_eth = sell − buy`.** A POSITIVE value means the
    /// cohort **distributed** (took ETH out); negative means it accumulated.
    ///
    /// Cohorts are mutually exclusive and assigned by a priority ladder:
    /// `kol` → `bot` → `dump_cluster` → `early_buyer` → `unprofiled` →
    /// `smart_money` → `retail`. `smart_money` is derived (win-rate ≥ 0.5 and net
    /// positive), and `unprofiled` is a real answer — that trader has not met the
    /// reputation thresholds. There is no `fresh_wallet` cohort: Robinhood Chain
    /// stores no wallet-level first-seen.
    pub async fn flow(&self, address: &str, params: &FlowParams) -> Result<serde_json::Value> {
        self.core
            .get(&format!("/rhc/tokens/{}/flow", address), params)
            .await
    }

    /// Peak MC, drawdown and high-water curve
    /// (`GET /rhc/tokens/{address}/peak-history`, PRO+).
    ///
    /// Returns **two peaks, because they disagree**. `peak_mc_usd_recorded` is the
    /// stored high-water mark every other Robinhood Chain surface keys off
    /// (deployer runner-rate, the $40K graduation bar); it is sampled from write
    /// batches, so it can undercount an intra-batch spike.
    /// `peak_mc_usd_observed` is the max of 1-minute candle highs — trade-level
    /// truth, and always ≥ recorded.
    ///
    /// Candle history begins 2026-07-15, so check
    /// `observed_covers_full_history` before treating the observed figure as a
    /// lifetime maximum.
    pub async fn peak_history(
        &self,
        address: &str,
        params: &PeakHistoryParams,
    ) -> Result<serde_json::Value> {
        self.core
            .get(&format!("/rhc/tokens/{}/peak-history", address), params)
            .await
    }

    /// EVM-native risk, computed LIVE (`GET /rhc/tokens/{address}/risk`, PRO+).
    ///
    /// **Not a port of the Solana risk model.** EVM has no mint or freeze
    /// authority: across 300 random Robinhood Chain tokens only 2.3% even expose
    /// an owner function and 0% expose `mint` in their own bytecode, so an absent
    /// capability flag is the **norm**, not a safety signal.
    ///
    /// The discriminating signals are proxy upgradeability, LP custody and above
    /// all **sellability**, which is simulated at the chain head through the
    /// router and is never cached — whether a token can be sold changes the moment
    /// an owner flips a setting. Note `owner.model = "none"` (no owner function at
    /// all) is a different answer from `"renounced"`, and `lp_custody` is read only
    /// for uniswap-v2 pools (v3/v4 LP sits in an NFT and reports `"unknown"`).
    pub async fn risk(&self, address: &str) -> Result<serde_json::Value> {
        self.core
            .get(&format!("/rhc/tokens/{}/risk", address), &())
            .await
    }

    /// Exact holder set + concentration (`GET /rhc/tokens/{address}/holders`, PRO+).
    ///
    /// Balances are folded from ERC-20 `Transfer` logs — **not** derived from
    /// trades — and reconciled against on-chain `totalSupply()` at a pinned block.
    ///
    /// **Check `verified` first.** `false` means the reconstruction is incomplete
    /// for that token and `unverified_reason` says why. Concentration **excludes
    /// liquidity pools and burn addresses** from the circulating denominator (the
    /// largest holder is otherwise the token's own pool) and reports them as
    /// `pool_held_pct` / `burned_pct`. `balance` is a raw uint256 returned as a
    /// decimal **string**. Holder addresses may be ERC-4337 smart accounts, so
    /// `holder_count` is not a headcount of people.
    ///
    /// `holder_growth.{1h,24h,7d}` reports `entered` (first `Transfer` at-or-after
    /// the window's `cutoff_block`), `entered_still_holding`, `exited` (pre-existing
    /// holders whose last `Transfer` in the window left them at zero) and `net` ≈
    /// Δ `holder_count` — typed as [`crate::types::HolderGrowth`]:
    /// `serde_json::from_value::<Option<HolderGrowth>>(resp["holder_growth"].clone())`.
    /// A window is `null` only when the chain had no ingested trades in it; the
    /// object is `null` only if the growth read failed.
    pub async fn holders(
        &self,
        address: &str,
        params: &HoldersParams,
    ) -> Result<serde_json::Value> {
        self.core
            .get(&format!("/rhc/tokens/{}/holders", address), params)
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
