use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Robinhood Chain price alerts — push when a token you picked drops, and
/// optionally again when it bounces back.
///
/// An alert is a **delta from the moment you set it**: the baseline market cap
/// is captured at creation, so the token must already be tracked on RHC with a
/// market cap. Alerts self-expire 30 days after creation.
///
/// ⚠️ **RHC alerts are POLLED (~15s off `rhc_token_prices`), NOT sub-second like
/// the Solana price alerts** — the RHC price writer emits no `pg_notify`, so
/// there is no live price loop to react to. Effective latency is that poll
/// interval plus the token's own price-update cadence; every create response
/// spells this out in its [`PriceAlertEvaluation`] block.
///
/// ⚠️ **Quotas are PER CHAIN**, and count only ACTIVE alerts.
///
/// Every method here is **PRO+**.
#[derive(Debug, Clone)]
pub struct PriceAlerts {
    pub(crate) core: Arc<HttpCore>,
}

impl PriceAlerts {
    /// Your price alerts (`GET /rhc/price-alerts`, PRO+).
    ///
    /// Newest first. Quota is per chain — Solana alerts do not count against it.
    pub async fn list(&self) -> Result<PriceAlertListResponse> {
        self.core.get("/rhc/price-alerts", &()).await
    }

    /// Create a price alert (`POST /rhc/price-alerts`, PRO+).
    ///
    /// The baseline market cap is captured **now**, so the token must already be
    /// tracked on Robinhood Chain with a market cap (a 400 otherwise). Omit
    /// `recovery_pct` for a dip-only, terminal alert.
    ///
    /// The response carries `webhook_secret` **once** (minted whenever a
    /// `webhook_url` was supplied) plus the [`PriceAlertEvaluation`] block
    /// stating the ~15s polled evaluation mode.
    ///
    /// ```no_run
    /// # use robinhood_chain::types::*;
    /// # async fn run(client: robinhood_chain::RobinhoodChain) -> Result<(), Box<dyn std::error::Error>> {
    /// let created = client.price_alerts.create(&PriceAlertCreateParams {
    ///     token_address: "0xabc…".into(),
    ///     drop_pct: 30.0,
    ///     recovery_pct: Some(20.0),
    ///     delivery_mode: Some(DeliveryMode::Websocket),
    ///     ..Default::default()
    /// }).await?;
    /// println!("{} every {}s", created.evaluation.mode, created.evaluation.interval_seconds);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        params: &PriceAlertCreateParams,
    ) -> Result<PriceAlertCreateResponse> {
        self.core.post("/rhc/price-alerts", params).await
    }

    /// Fetch one price alert (`GET /rhc/price-alerts/{id}`, PRO+).
    ///
    /// `id` is the numeric alert id. Returns a 404
    /// [`Error::Api`](crate::Error) when you do not own it.
    pub async fn get(&self, id: i64) -> Result<PriceAlertGetResponse> {
        self.core
            .get(&format!("/rhc/price-alerts/{}", id), &())
            .await
    }

    /// Update a price alert (`PATCH /rhc/price-alerts/{id}`, PRO+).
    ///
    /// Only `name`, `delivery_mode`, `webhook_url` and `is_active` are mutable —
    /// retuning `token_address`, `drop_pct` or `recovery_pct` mid-flight would
    /// make the alert's already-recorded events uninterpretable, so those are
    /// rejected with a 400. Delete and recreate instead.
    pub async fn update(
        &self,
        id: i64,
        params: &PriceAlertUpdateParams,
    ) -> Result<PriceAlertGetResponse> {
        self.core
            .patch(&format!("/rhc/price-alerts/{}", id), params)
            .await
    }

    /// Delete a price alert (`DELETE /rhc/price-alerts/{id}`, PRO+).
    ///
    /// Its events cascade. Deleting an alert you do not own is a 404.
    pub async fn delete(&self, id: i64) -> Result<RhcDeletedResponse> {
        self.core
            .delete(&format!("/rhc/price-alerts/{}", id))
            .await
    }

    /// Dip and recovery events for your price alerts
    /// (`GET /rhc/price-alerts/events`, PRO+).
    ///
    /// The catch-up path for a missed webhook or a dropped WS channel — newest
    /// `fired_at` first, retained **30 days**. Poll with `since` set to the
    /// newest `fired_at` you have already processed.
    pub async fn events(
        &self,
        params: &PriceAlertEventsParams,
    ) -> Result<PriceAlertEventsResponse> {
        self.core.get("/rhc/price-alerts/events", params).await
    }
}
