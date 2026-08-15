use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Wallet intelligence and the per-chain wallet watchlist.
///
/// [`profile`](Wallet::profile), [`pnl`](Wallet::pnl) and
/// [`positions`](Wallet::positions) are served from ONE shared 90-day snapshot
/// cache, so calling all three on the same address costs roughly one
/// computation rather than three — `cache_hit` says which call paid for it.
///
/// ⚠️ Every amount is **ETH**, not SOL and not USD.
///
/// ⚠️ Cost basis is **FIFO over a rolling 90-day window**. "Open" means
/// FIFO-unmatched buys inside that window, so a position opened before the
/// window reads as a sell with no matching buy — `cost_basis_observable_from`
/// on the PnL response states the exact date that applies.
///
/// Every method here is **PRO+**.
#[derive(Debug, Clone)]
pub struct Wallet {
    pub(crate) core: Arc<HttpCore>,
}

impl Wallet {
    /// A wallet's 90-day trading profile (`GET /rhc/wallet/{address}`, PRO+).
    ///
    /// FIFO cost-basis PnL, per-token breakdown, recent trades, and a
    /// reputation block: tracked KOL, known deployer + tier, alpha-ranked,
    /// dump-cluster membership, early-buyer count.
    ///
    /// `stats.unattributed_trades` counts pre-2026-07-18 rows whose
    /// `trader_eoa` is NULL. Those are unattributable by design and excluded
    /// from every PnL figure, so a low `analyzed_trades` on an old wallet is a
    /// data-window limit, not inactivity.
    ///
    /// `stats_unavailable` means the snapshot timed out; `flags` still resolve.
    pub async fn profile(&self, address: &str) -> Result<WalletProfileResponse> {
        self.core.get(&format!("/rhc/wallet/{address}"), &()).await
    }

    /// Full FIFO cost-basis PnL (`GET /rhc/wallet/{address}/pnl`, PRO+).
    ///
    /// Realized and unrealized split, a daily realized curve, every closed
    /// position with ROI and hold time, and every open position marked to the
    /// current price. This runs the **same FIFO implementation** as the Solana
    /// wallet PnL endpoint, so the two chains are directly comparable.
    ///
    /// Read `notes.partial` before quoting any total.
    pub async fn pnl(&self, address: &str) -> Result<WalletPnlResponse> {
        self.core.get(&format!("/rhc/wallet/{address}/pnl"), &()).await
    }

    /// Open positions only (`GET /rhc/wallet/{address}/positions`, PRO+).
    ///
    /// The same FIFO pass as [`pnl`](Wallet::pnl) without the curve and closed
    /// positions — for clients polling "what is this wallet in right now".
    ///
    /// ⚠️ Check `positions[].liquidity_basis`: `v4_virtual_ceiling` means
    /// `liquidity_usd` is a bonding-curve virtual ceiling, **not** withdrawable
    /// TVL. Never size an exit against it.
    pub async fn positions(&self, address: &str) -> Result<WalletPositionsResponse> {
        self.core.get(&format!("/rhc/wallet/{address}/positions"), &()).await
    }

    /// One wallet's trade tape (`GET /rhc/wallet/{address}/trades`, PRO+).
    ///
    /// Newest first, cursor-paginated on the opaque `next_before` keyset.
    /// Distinct from [`Trades::list`](crate::api::trades::Trades::list) with a
    /// `token` filter: that filters the global tape by TOKEN, this filters by
    /// WALLET, which is a different index path.
    pub async fn trades(
        &self,
        address: &str,
        params: &WalletTradesParams,
    ) -> Result<WalletTradesResponse> {
        self.core.get(&format!("/rhc/wallet/{address}/trades"), params).await
    }

    /// Your Robinhood Chain watchlist
    /// (`GET /rhc/wallet-tracker/watchlist`, PRO+).
    ///
    /// Quotas are **per chain** — PRO 50 / ULTRA 100 / BUSINESS 500 RHC
    /// wallets, independent of your Solana watchlist, so adopting Robinhood
    /// Chain never shrinks an existing Solana list.
    pub async fn watchlist(&self) -> Result<WalletTrackerListResponse> {
        self.core.get("/rhc/wallet-tracker/watchlist", &()).await
    }

    /// Track a wallet (`POST /rhc/wallet-tracker/watchlist`, PRO+).
    ///
    /// The address is stored lowercase so it matches `rhc_trades.trader_eoa` —
    /// a checksummed `0xAbC…` would join to nothing and the wallet would look
    /// permanently silent.
    ///
    /// # Errors
    ///
    /// 409 [`Error::Api`](crate::Error) if the wallet is already tracked, 403
    /// once you are at your tier cap.
    pub async fn track(
        &self,
        params: &WalletTrackerAddParams,
    ) -> Result<WalletTrackerWalletResponse> {
        self.core.post("/rhc/wallet-tracker/watchlist", params).await
    }

    /// Untrack a wallet
    /// (`DELETE /rhc/wallet-tracker/watchlist/{address}`, PRO+).
    ///
    /// Frees one slot against your per-chain quota.
    ///
    /// # Errors
    ///
    /// 404 [`Error::Api`](crate::Error) if the wallet is not on your list.
    pub async fn untrack(&self, address: &str) -> Result<WalletTrackerRemovedResponse> {
        self.core.delete(&format!("/rhc/wallet-tracker/watchlist/{address}")).await
    }

    /// Relabel a tracked wallet
    /// (`PATCH /rhc/wallet-tracker/watchlist/{address}`, PRO+).
    ///
    /// Pass `None` to clear the label — `null` is accepted here, unlike on
    /// [`track`](Wallet::track), where the field must be omitted instead.
    pub async fn relabel(
        &self,
        address: &str,
        label: Option<&str>,
    ) -> Result<WalletTrackerWalletResponse> {
        #[derive(serde::Serialize)]
        struct Body<'a> {
            label: Option<&'a str>,
        }
        self.core
            .patch(&format!("/rhc/wallet-tracker/watchlist/{address}"), &Body { label })
            .await
    }

    /// Merged tape across your tracked wallets
    /// (`GET /rhc/wallet-tracker/trades`, PRO+).
    ///
    /// Every trade by every wallet on your watchlist, newest first, each row
    /// tagged with its watchlist label. The cursor (`next_before`) is an opaque
    /// keyset matching the rest of the Robinhood Chain tree, **not** the Solana
    /// tracker's integer epoch. A `wallet` filter must already be tracked.
    pub async fn tracked_trades(
        &self,
        params: &WalletTrackerTradesParams,
    ) -> Result<WalletTrackerTradesResponse> {
        self.core.get("/rhc/wallet-tracker/trades", params).await
    }

    /// Per-wallet rollup across your tracked wallets
    /// (`GET /rhc/wallet-tracker/summary`, PRO+).
    ///
    /// Buy/sell/volume per wallet over the chosen period. Sourced from
    /// `rhc_trades` **directly**, not from a per-subscriber capture log: on
    /// Robinhood Chain every swap is already recorded, so a newly tracked
    /// wallet has its full history immediately rather than only from the moment
    /// you started tracking it. The Solana tracker cannot do this, because
    /// there trades are only captured for wallets somebody asked for.
    ///
    /// `stats_unavailable` means the rollup timed out and the per-wallet stats
    /// are zeroed, not absent.
    pub async fn tracked_summary(
        &self,
        params: &WalletTrackerSummaryParams,
    ) -> Result<WalletTrackerSummaryResponse> {
        self.core.get("/rhc/wallet-tracker/summary", params).await
    }
}
