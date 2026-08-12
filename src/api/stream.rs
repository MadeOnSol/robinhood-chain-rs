use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// WebSocket channel for live Robinhood Chain KOL trades.
///
/// Subscribe to this after connecting to [`StreamToken::ws_url`] to receive
/// every tracked-KOL buy/sell on chain 4663 the moment it lands (event
/// `rhc:kol_trade`). No channel-level tier gate — any tier that can hold a
/// stream connection may subscribe (the stream token itself is PRO+).
pub const RHC_KOL_TRADES: &str = "rhc:kol_trades";

/// WebSocket channel for the full Robinhood Chain DEX trade firehose (ULTRA+).
///
/// Every fresh attributed Uniswap v2/v3/v4 swap on chain 4663, ~40-55/s at
/// tip (event `rhc:dex_trade`). Gated ULTRA/BUSINESS — a lower-tier subscribe
/// is answered with a `channels_rejected` warning frame (see the
/// [module docs](self)), not an error.
pub const RHC_DEX_TRADES: &str = "rhc:dex_trades";

/// Renamed — use [`RHC_DEX_TRADES`].
///
/// 0.4.0 shipped this constant as `"rhc:trades"`, a channel that never existed
/// server-side, so every subscribe drew a `channels_rejected` warning and then
/// silence. The value now carries the real channel name `"rhc:dex_trades"`
/// (the server also accepts `rhc:trades` as a deprecated alias, acked under
/// the canonical name). Note the real gate is ULTRA+, not PRO+ as 0.4.0's docs
/// claimed.
#[deprecated(note = "renamed; use RHC_DEX_TRADES")]
pub const RHC_TRADES: &str = "rhc:dex_trades";

/// WebSocket channel for copy-trade rule fires (PRO+).
///
/// One `rhc:copytrade:signal` event each time one of **your** copy-trade rules
/// (`client.copytrade`) fires. User-scoped: a rule's signals are only ever
/// delivered to the rule's owner, never to other subscribers of the channel.
pub const RHC_COPYTRADE_SIGNALS: &str = "rhc:copytrade:signals";

/// WebSocket channel for price-alert rule fires (PRO+).
///
/// `rhc:price_alert:dip` and `rhc:price_alert:recovery` events for **your**
/// price alerts (`client.price_alerts`). User-scoped, like copy-trade. RHC
/// price alerts are ~15s **polled**, not sub-second — expect that interval
/// plus the token's own price-update cadence.
pub const RHC_PRICE_ALERT_EVENTS: &str = "rhc:price_alert:events";

/// WebSocket channel for KOL coordination-alert rule fires (PRO+).
///
/// One `rhc:kol:coordination` event when one of **your** coordination-alert
/// rules (`client.kol.coordination_alerts_*`) fires — N+ tracked KOLs buying
/// the same token inside the rule's window. User-scoped.
pub const RHC_KOL_COORDINATION: &str = "rhc:kol:coordination";

/// WebSocket channel for KOL first-touch events (PRO+).
///
/// One `rhc:kol:first_touch` event when a token receives its FIRST
/// tracked-KOL buy ever. **Broadcast, not user-scoped** — every subscriber
/// sees every first touch, and the channel is PRO+. ULTRA gates only the
/// first-touch *subscription* CRUD (`client.kol.first_touch_subscriptions_*`,
/// per-rule filtered webhooks), not this channel.
pub const RHC_KOL_FIRST_TOUCHES: &str = "rhc:kol:first_touches";

/// WebSocket streaming: issue a token, list live sessions, evict a session.
///
/// Connect to [`StreamToken::ws_url`] with `?token=<token>` appended, then
/// subscribe to the Robinhood Chain channels — the two read channels
/// [`RHC_KOL_TRADES`] and [`RHC_DEX_TRADES`] (ULTRA+), plus the four
/// rule-engine channels [`RHC_COPYTRADE_SIGNALS`], [`RHC_PRICE_ALERT_EVENTS`],
/// [`RHC_KOL_COORDINATION`] and [`RHC_KOL_FIRST_TOUCHES`]. The same
/// `wss://madeonsol.com/stream` protocol as the Solana stream client.
///
/// # Rejected channels are a warning frame, not an error
///
/// The server never fails a `subscribe` message outright. Channels your tier
/// cannot access (or names it does not recognize) are dropped and reported in
/// a warning frame, followed by the normal `subscribed` ack listing what
/// actually took effect:
///
/// ```json
/// {
///   "type": "warning",
///   "code": "channels_rejected",
///   "rejected": [ { "channel": "rhc:dex_trades", "reason": "requires ULTRA" } ],
///   "valid_channels": ["kol:trades", "...", "rhc:kol:first_touches"],
///   "ts": 1723400000000
/// }
/// ```
///
/// This crate does not include a WebSocket client — you connect with your own
/// (e.g. `tokio-tungstenite`) and receive server frames raw. Watch for
/// `type: "warning"` frames: a rejected channel otherwise looks like a healthy
/// but silent subscription.
#[derive(Debug, Clone)]
pub struct Stream {
    pub(crate) core: Arc<HttpCore>,
}

impl Stream {
    /// Generate a 24-hour WebSocket streaming token (`POST /stream/token`, PRO+).
    ///
    /// Returns `ws_url` for KOL/DEX event streaming; ULTRA also returns
    /// `dex_ws_url` for the all-DEX firehose. Connect by appending
    /// `?token=<token>` to the URL, then subscribe to [`RHC_KOL_TRADES`],
    /// [`RHC_DEX_TRADES`] (ULTRA+), or the four rule-engine channels.
    pub async fn get_token(&self) -> Result<StreamToken> {
        self.core.post_empty("/stream/token").await
    }

    /// List your live WebSocket sessions (`GET /stream/sessions`, PRO+).
    pub async fn sessions(&self) -> Result<StreamSessionsResponse> {
        self.core.get("/stream/sessions", &()).await
    }

    /// Force-disconnect one of your live WebSocket sessions by id
    /// (`DELETE /stream/sessions/{id}`, PRO+), freeing its connection slot.
    /// Returns `{ evicted: true, id }`.
    pub async fn kill_session(&self, id: &str) -> Result<StreamSessionEvicted> {
        self.core
            .delete(&format!("/stream/sessions/{}", id))
            .await
    }
}
