//! # robinhood-chain — Robinhood Chain SDK for Rust
//!
//! EVM-native on-chain trading intelligence for **Robinhood Chain (chain id 4663)**:
//! live KOL trades, token discovery & launch-bundle detection, deployer reputation,
//! smart-money wallet ranking, OHLC candles, and the DEX trade tape — all from our
//! self-hosted node.
//!
//! Robinhood Chain is an Arbitrum Orbit L2, so every field is EVM-native:
//! `token_address` (lowercase `0x…`), `eth_amount`, `tx_hash`, `block_number`,
//! `net_flow_eth`. There are no Solana field names here.
//!
//! ## Get an API key
//!
//! Robinhood Chain coverage is **bundled into every MadeOnSol tier at no extra cost** —
//! same `msk_` key, same base URL. Get a free key at <https://madeonsol.com/pricing>.
//! Paid tiers (PRO / ULTRA) unlock the DEX trade tape, token discovery, candles,
//! KOL-consensus, alpha-wallet ranking, and WebSocket streaming — and new customers
//! get a **3-day free trial** of Pro or Ultra when paying by card. See
//! <https://madeonsol.com/pricing>.
//!
//! ## Quick start
//!
//! ```no_run
//! use robinhood_chain::{RobinhoodChain, types::KolFeedParams};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let api_key = std::env::var("MADEONSOL_API_KEY")?;
//! let client = RobinhoodChain::new(api_key)?;
//!
//! let feed = client
//!     .kol
//!     .feed(&KolFeedParams { limit: Some(10), ..Default::default() })
//!     .await?;
//!
//! for trade in feed.trades {
//!     println!("{:?} {:?} {:?} ({} ETH)",
//!         trade.kol_name, trade.action, trade.token_symbol,
//!         trade.eth_amount.unwrap_or(0.0));
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Namespaces
//!
//! - [`RobinhoodChain::kol`] — KOL feed, leaderboard, consensus hot-tokens, coordination, first-touches, single-KOL profile, plus the coordination-alert (PRO+) and first-touch-subscription (ULTRA+) rule engines
//! - [`RobinhoodChain::trades`] — the DEX trade tape (PRO+) + the liquidity-removals feed `lp_events` (PRO+, removals only)
//! - [`RobinhoodChain::tokens`] — token discovery, beacon-verified tokenized equities, per-token snapshot, candles, KOL-consensus, buyer-quality, bundle, batch reads
//! - [`RobinhoodChain::deployer_hunter`] — deployer reputation: leaderboard, profile, trajectory, launch history, best-tokens, stats, alerts, recent graduations
//! - [`RobinhoodChain::alpha_wallets`] — smart-money wallet ranking (PRO+)
//! - [`RobinhoodChain::wallet`] — wallet profile, FIFO PnL, positions, tape, watchlist (PRO+)
//! - [`RobinhoodChain::copytrade`] — copy-trade rules + fired-signal history (PRO+)
//! - [`RobinhoodChain::price_alerts`] — price alerts + dip/recovery events (PRO+)
//! - [`RobinhoodChain::stream`] — WebSocket streaming token issuance + the six `rhc:*` channels (`rhc:kol_trades`, `rhc:dex_trades` (ULTRA+), and the four rule-engine channels)
//!
//! ## Push rule engines
//!
//! Four rule engines turn the read endpoints into push: copy-trade, price
//! alerts, KOL coordination and KOL first-touches. Each rule delivers by
//! webhook (HMAC-SHA256 signed), WebSocket, or both, and each keeps a
//! queryable fire history for catch-up after a missed delivery.
//!
//! ⚠️ **Quotas are PER CHAIN** — a full set of Solana rules does not consume any
//! Robinhood Chain capacity. Two RHC-specific differences worth knowing before
//! you port Solana code: RHC price alerts are **~15s polled, not sub-second**,
//! and RHC copy-trade rules have **no market-cap band**.
//!
//! Full API reference: <https://madeonsol.com/api-docs> · Robinhood Chain overview:
//! <https://madeonsol.com/robinhood>

#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]

mod client;
pub mod api;
pub mod error;
pub mod types;

use std::sync::Arc;

use crate::api::{
    alpha_wallets::AlphaWallets, copytrade::CopyTrade, deployer_hunter::DeployerHunter, kol::Kol,
    price_alerts::PriceAlerts, stream::Stream, tokens::Tokens, trades::Trades,
    wallet::Wallet,
};
use crate::client::HttpCore;
use crate::error::{Result, RobinhoodChainError};

pub use crate::error::RobinhoodChainError as Error;

/// Robinhood Chain API client.
///
/// Construct with [`RobinhoodChain::new`] and a `msk_…` API key, then access the
/// namespaced sub-clients ([`kol`](Self::kol), [`tokens`](Self::tokens), etc.).
///
/// Cheap to clone — internal HTTP state is reference-counted.
///
/// # Example
///
/// ```no_run
/// use robinhood_chain::RobinhoodChain;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = RobinhoodChain::new(std::env::var("MADEONSOL_API_KEY")?)?;
/// let feed = client.kol.feed(&Default::default()).await?;
/// println!("{} recent KOL trades on chain {}", feed.count, feed.chain);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct RobinhoodChain {
    /// KOL trade intelligence: feed, leaderboard, consensus hot-tokens,
    /// coordination, first-touches, profile.
    pub kol: Kol,
    /// The Robinhood Chain DEX trade tape + liquidity-removals feed (PRO+).
    pub trades: Trades,
    /// Token intelligence: discovery, beacon-verified tokenized equities,
    /// snapshot, candles, KOL-consensus, buyer-quality, bundle, and the two
    /// batch reads.
    pub tokens: Tokens,
    /// Deployer reputation: leaderboard, profile, trajectory, launch history,
    /// best-tokens, chain-wide stats, alerts, recent graduations.
    pub deployer_hunter: DeployerHunter,
    /// Smart-money wallet ranking (PRO+).
    pub alpha_wallets: AlphaWallets,
    /// Wallet intelligence: 90-day ETH profile, FIFO PnL, open positions,
    /// per-wallet tape, and the per-chain wallet watchlist (PRO+).
    pub wallet: Wallet,
    /// Copy-trade rule engine: rules + fired-signal history (PRO+).
    pub copytrade: CopyTrade,
    /// Price-alert rule engine: alerts + dip/recovery events (PRO+).
    pub price_alerts: PriceAlerts,
    /// WebSocket streaming token issuance + the six `rhc:*` channels
    /// (`rhc:kol_trades`, `rhc:dex_trades` (ULTRA+), and the four rule-engine
    /// channels — see [`api::stream`]).
    pub stream: Stream,
}

impl RobinhoodChain {
    /// Construct a new client.
    ///
    /// `api_key` must start with `msk_`. Robinhood Chain coverage is bundled into
    /// every tier — get a free key at <https://madeonsol.com/pricing>.
    ///
    /// # Errors
    ///
    /// Returns [`RobinhoodChainError::MissingApiKey`] if the key is empty or
    /// missing the `msk_` prefix.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let api_key = api_key.into();
        if !api_key.starts_with("msk_") {
            eprintln!(
                "\n[robinhood-chain] Missing or invalid API key.\n\
                 → Get a free key at https://madeonsol.com/pricing (RHC bundled into every tier)\n\
                 → Then: robinhood_chain::RobinhoodChain::new(std::env::var(\"MADEONSOL_API_KEY\")?)?\n"
            );
            return Err(RobinhoodChainError::MissingApiKey);
        }

        let core = Arc::new(HttpCore::new(api_key));
        Ok(Self {
            kol: Kol { core: Arc::clone(&core) },
            trades: Trades { core: Arc::clone(&core) },
            tokens: Tokens { core: Arc::clone(&core) },
            deployer_hunter: DeployerHunter { core: Arc::clone(&core) },
            alpha_wallets: AlphaWallets { core: Arc::clone(&core) },
            wallet: Wallet { core: Arc::clone(&core) },
            copytrade: CopyTrade { core: Arc::clone(&core) },
            price_alerts: PriceAlerts { core: Arc::clone(&core) },
            stream: Stream { core },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_missing_api_key() {
        let err = RobinhoodChain::new("").unwrap_err();
        assert!(matches!(err, RobinhoodChainError::MissingApiKey));
    }

    #[test]
    fn rejects_wrong_prefix() {
        let err = RobinhoodChain::new("sk_live_abc").unwrap_err();
        assert!(matches!(err, RobinhoodChainError::MissingApiKey));
    }

    #[test]
    fn accepts_valid_prefix() {
        let client = RobinhoodChain::new("msk_test_abcdef").unwrap();
        // Smoke test — namespaces exist and the client clones cheaply.
        let _cloned = client.clone();
    }

    /// An all-default PATCH body must serialize to `{}` — never to a wall of
    /// nulls that would clear every nullable column.
    #[test]
    fn patch_omits_untouched_fields() {
        let body = serde_json::to_string(&types::CopyTradeUpdateParams::default()).unwrap();
        assert_eq!(body, "{}");
    }

    /// `Option<Option<T>>` must distinguish "leave alone" from "set null".
    #[test]
    fn patch_distinguishes_omit_from_explicit_null() {
        let clear = serde_json::to_value(&types::PriceAlertUpdateParams {
            name: Some(None),
            is_active: Some(false),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(clear["name"], serde_json::Value::Null);
        assert!(clear.get("name").is_some(), "explicit null must be sent");
        assert!(clear.get("webhook_url").is_none(), "omitted key must not be sent");

        let set = serde_json::to_value(&types::PriceAlertUpdateParams {
            name: Some(Some("renamed".into())),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(set["name"], "renamed");
    }

    /// Rule-engine enums must hit the exact wire literals the API validates on.
    #[test]
    fn rule_engine_enums_match_wire_literals() {
        use types::*;
        assert_eq!(
            serde_json::to_string(&DeliveryMode::Websocket).unwrap(),
            "\"websocket\""
        );
        assert_eq!(
            serde_json::to_string(&CopyTradeSizingMode::PercentSource).unwrap(),
            "\"percent_source\""
        );
        assert_eq!(
            serde_json::to_string(&FirstTouchStrategy::DayTrader).unwrap(),
            "\"day_trader\""
        );
        assert_eq!(DeliveryMode::Both.as_str(), "both");
        assert_eq!(PriceAlertStatus::Watching.as_str(), "watching");
        assert_eq!(PriceAlertEventType::Recovery.as_str(), "recovery");
        assert_eq!(CopyTradeOnlyAction::Sell.as_str(), "sell");
    }

    /// The two fire-history endpoints omit `count` entirely when the caller owns
    /// no rules at all — that must not be a parse failure.
    #[test]
    fn fire_history_parses_without_count() {
        let signals: types::CopyTradeSignalsResponse =
            serde_json::from_str(r#"{"chain":"robinhood","signals":[]}"#).unwrap();
        assert_eq!(signals.count, 0);

        let events: types::PriceAlertEventsResponse =
            serde_json::from_str(r#"{"chain":"robinhood","events":[]}"#).unwrap();
        assert_eq!(events.count, 0);
    }

    /// A v4 liquidity removal carries `liquidity` only — the amount fields are
    /// null on the wire and must land as `None`, never as a parse failure. Raw
    /// amounts stay decimal strings (uint256 does not fit an f64).
    #[test]
    fn lp_event_v4_row_parses_with_null_amounts() {
        let resp: types::LpEventsResponse = serde_json::from_str(
            r#"{"chain":"robinhood","events":[{"event":"remove","pool":"0xabc","dex":"uniswap-v4",
                "fee_tier":null,"token_address":"0x1111111111111111111111111111111111111111",
                "token_symbol":null,"token_name":null,"token_decimals":18,"launchpad":null,
                "provider":"0x2222222222222222222222222222222222222222","provider_is_token_deployer":true,
                "provider_deployer_tier":null,"provider_kol_name":null,
                "liquidity":"340282366920938463463374607431768211455","amount0":null,"amount1":null,
                "token0":null,"token1":null,"token_amount_raw":null,"quote_token":null,"quote_amount_raw":null,
                "block_number":123,"block_time":"2026-08-16T00:00:00Z","tx_hash":"0xdead","log_index":4}],
                "count":1,"has_more":false,"next_before":null,
                "coverage":{"events":["remove"],"adds_persisted":false,"note":"x","since":"2026-08-05"}}"#,
        )
        .unwrap();
        let ev = &resp.events[0];
        assert_eq!(ev.event, "remove");
        assert!(ev.provider_is_token_deployer);
        assert_eq!(ev.liquidity.as_deref(), Some("340282366920938463463374607431768211455"));
        assert!(ev.amount0.is_none() && ev.token_amount_raw.is_none());
        assert!(!resp.coverage.as_ref().unwrap().adds_persisted);
        assert_eq!(
            serde_json::to_string(&types::LpEventsParams { dex: Some(types::TradeDex::UniswapV4), ..Default::default() }).unwrap(),
            r#"{"dex":"uniswap-v4"}"#
        );
    }

    /// Equities sort keys hit the exact wire literals; an unpriced equity parses
    /// with `None` numerics and zeroed 24h counters.
    #[test]
    fn equities_sort_and_unpriced_row() {
        assert_eq!(serde_json::to_string(&types::EquitiesSort::MarketCap).unwrap(), "\"market_cap\"");
        assert_eq!(types::EquitiesSort::LastTrade.as_str(), "last_trade");
        let resp: types::EquitiesResponse = serde_json::from_str(
            r#"{"chain":"robinhood","equities":[{"token_address":"0x3333333333333333333333333333333333333333",
                "symbol":"NVDA","name":"NVIDIA","onchain_name":"NVIDIA • Robinhood Token","asset_class":"equity",
                "verified":true,"issuer_beacon":"0xe10b6f6b275de231345c20d14ab812db62151b00","decimals":18,
                "listed_at":null,"price_usd":null,"price_native":null,"market_cap_usd":null,"fdv_usd":null,
                "peak_mc_usd":null,"liquidity_usd":null,"liquidity_basis":null,"primary_dex":null,
                "primary_pool":null,"last_trade_time":null,"trades_24h":0,"volume_eth_24h":0,"buys_24h":0,
                "sells_24h":0,"buyers_24h":0,"sellers_24h":0}],
                "count":1,"total_equities":157,"sort":"volume",
                "identity":{"method":"beacon","issuer_beacon":"0xe10b6f6b275de231345c20d14ab812db62151b00","note":"n"},
                "stats_window":"24h","stats_as_of":"2026-08-16T00:00:00Z"}"#,
        )
        .unwrap();
        let e = &resp.equities[0];
        assert_eq!(e.symbol.as_deref(), Some("NVDA"));
        assert!(e.verified && e.price_usd.is_none());
        assert_eq!(resp.total_equities, 157);
        assert_eq!(resp.identity.as_ref().unwrap().method.as_deref(), Some("beacon"));
    }

    /// A first-touch subscription with no filters comes back as `{}`, and an
    /// empty filter set must serialize to `{}` rather than a null soup.
    #[test]
    fn first_touch_filters_round_trip() {
        let sub: types::RhcFirstTouchSubscription = serde_json::from_str(
            r#"{"id":"11111111-1111-1111-1111-111111111111","name":null,"filters":{},
                "delivery_mode":"websocket","webhook_url":null,"is_active":true,
                "created_at":"2026-08-01T00:00:00Z","updated_at":"2026-08-01T00:00:00Z"}"#,
        )
        .unwrap();
        assert!(sub.filters.kol.is_none());
        assert_eq!(sub.delivery_mode, types::DeliveryMode::Websocket);
        assert_eq!(
            serde_json::to_string(&types::FirstTouchFilters::default()).unwrap(),
            "{}"
        );
    }
}
