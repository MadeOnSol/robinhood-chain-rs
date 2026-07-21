use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// WebSocket channel for live Robinhood Chain KOL trades.
///
/// Subscribe to this after connecting to [`StreamToken::ws_url`] to receive
/// every tracked-KOL buy/sell on chain 4663 the moment it lands.
pub const RHC_KOL_TRADES: &str = "rhc:kol_trades";

/// WebSocket channel for the full Robinhood Chain DEX trade firehose.
///
/// Subscribe to this after connecting to [`StreamToken::ws_url`] to receive
/// every Uniswap v2/v3/v4 swap on chain 4663 (PRO+).
pub const RHC_TRADES: &str = "rhc:trades";

/// WebSocket streaming: issue a token, list live sessions, evict a session.
///
/// Connect to [`StreamToken::ws_url`] with `?token=<token>` appended, then
/// subscribe to the Robinhood Chain channels [`RHC_KOL_TRADES`] and
/// [`RHC_TRADES`]. The same `wss://madeonsol.com/stream` protocol as the Solana
/// stream client.
#[derive(Debug, Clone)]
pub struct Stream {
    pub(crate) core: Arc<HttpCore>,
}

impl Stream {
    /// Generate a 24-hour WebSocket streaming token (`POST /stream/token`, PRO+).
    ///
    /// Returns `ws_url` for KOL/DEX event streaming; ULTRA also returns
    /// `dex_ws_url` for the all-DEX firehose. Connect by appending
    /// `?token=<token>` to the URL, then subscribe to [`RHC_KOL_TRADES`] /
    /// [`RHC_TRADES`].
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
