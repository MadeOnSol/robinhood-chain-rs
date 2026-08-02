use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Robinhood Chain copy-trade rules — push when a wallet you follow trades.
///
/// A rule watches up to 250 `source_wallets` and fires on every qualifying RHC
/// swap they make, delivering a signal by webhook, WebSocket, or both.
///
/// ⚠️ **Quotas are PER CHAIN.** A full set of Solana copy-trade rules does not
/// consume any Robinhood Chain capacity, and vice versa.
///
/// ⚠️ Sizes are **ETH**, not SOL, and there is **no market-cap band** — the RHC
/// copy-trade notify payload carries no market cap, so a band could only be a
/// per-event DB lookup in the hot path of a ~3.3M-trades/day chain.
///
/// Every method here is **PRO+**.
#[derive(Debug, Clone)]
pub struct CopyTrade {
    pub(crate) core: Arc<HttpCore>,
}

impl CopyTrade {
    /// Your copy-trade rules (`GET /rhc/copytrade/subscriptions`, PRO+).
    ///
    /// Newest first. Quota is per chain — Solana rules do not count against it.
    pub async fn list(&self) -> Result<CopyTradeListResponse> {
        self.core.get("/rhc/copytrade/subscriptions", &()).await
    }

    /// Create a copy-trade rule (`POST /rhc/copytrade/subscriptions`, PRO+).
    ///
    /// Fires when one of `source_wallets` trades on Robinhood Chain. The wallet
    /// cap is per tier (PRO 5 / ULTRA 50 / BUSINESS 250) and the rule cap is
    /// per account (PRO 3 / ULTRA 20 / BUSINESS 100) — a breach is a 400/409
    /// [`Error::Api`](crate::Error).
    ///
    /// The response carries `webhook_secret` **once**; it is minted whenever a
    /// `webhook_url` was supplied and never shown again.
    ///
    /// ```no_run
    /// # use robinhood_chain::types::*;
    /// # async fn run(client: robinhood_chain::RobinhoodChain) -> Result<(), Box<dyn std::error::Error>> {
    /// let created = client.copytrade.create(&CopyTradeCreateParams {
    ///     name: Some("whale follow".into()),
    ///     source_wallets: vec!["0xabc…".into()],
    ///     sizing_amount: 0.05,
    ///     delivery_mode: Some(DeliveryMode::Websocket),
    ///     ..Default::default()
    /// }).await?;
    /// println!("rule {} — {}", created.subscription.id, created.note);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        params: &CopyTradeCreateParams,
    ) -> Result<CopyTradeCreateResponse> {
        self.core.post("/rhc/copytrade/subscriptions", params).await
    }

    /// Fetch one copy-trade rule
    /// (`GET /rhc/copytrade/subscriptions/{id}`, PRO+).
    ///
    /// `id` is the numeric rule id. Returns a 404
    /// [`Error::Api`](crate::Error) when you do not own it.
    pub async fn get(&self, id: i64) -> Result<CopyTradeGetResponse> {
        self.core
            .get(&format!("/rhc/copytrade/subscriptions/{}", id), &())
            .await
    }

    /// Partially update a copy-trade rule
    /// (`PATCH /rhc/copytrade/subscriptions/{id}`, PRO+).
    ///
    /// Omitted fields are left untouched; an empty body is a 400. The per-tier
    /// wallet cap is re-checked, so a PRO rule cannot be PATCHed past its limit.
    pub async fn update(
        &self,
        id: i64,
        params: &CopyTradeUpdateParams,
    ) -> Result<CopyTradeGetResponse> {
        self.core
            .patch(&format!("/rhc/copytrade/subscriptions/{}", id), params)
            .await
    }

    /// Delete a copy-trade rule
    /// (`DELETE /rhc/copytrade/subscriptions/{id}`, PRO+).
    ///
    /// Its fired signals cascade. Deleting a rule you do not own is a 404.
    pub async fn delete(&self, id: i64) -> Result<RhcDeletedResponse> {
        self.core
            .delete(&format!("/rhc/copytrade/subscriptions/{}", id))
            .await
    }

    /// Fire history for your copy-trade rules
    /// (`GET /rhc/copytrade/signals`, PRO+).
    ///
    /// The catch-up path when a webhook was missed or the WS channel dropped —
    /// newest `fired_at` first, retained **7 days**. Poll with `since` set to
    /// the newest `fired_at` you have already processed.
    pub async fn signals(
        &self,
        params: &CopyTradeSignalsParams,
    ) -> Result<CopyTradeSignalsResponse> {
        self.core.get("/rhc/copytrade/signals", params).await
    }
}
