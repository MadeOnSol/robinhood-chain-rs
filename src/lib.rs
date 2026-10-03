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

    /// Regression: since 2026-08-27 `POST /stream/token` returns
    /// `expires_at: null` / `next_refresh_at: null` (stream tokens never
    /// expire) plus `rotated` / `lifetime`. 0.8.0's
    /// `expires_at: String` refused that body, so `get_token()` errored for
    /// every caller.
    #[test]
    fn stream_token_deserializes_null_expiry() {
        let t: crate::types::StreamToken = serde_json::from_str(
            r#"{"token":"abc","expires_at":null,"next_refresh_at":null,"rotated":false,
                "lifetime":"This token does not expire.",
                "ws_url":"wss://madeonsol.com/ws/v1/stream","usage":"connect"}"#,
        )
        .unwrap();
        assert_eq!(t.token, "abc");
        assert!(t.expires_at.is_none());
        assert!(t.next_refresh_at.is_none());
        assert_eq!(t.rotated, Some(false));
        assert!(t.lifetime.is_some());
        assert!(t.dex_ws_url.is_none());

        // Pre-2026-08-27 servers sent a timestamp and omitted the new fields.
        let old: crate::types::StreamToken = serde_json::from_str(
            r#"{"token":"abc","expires_at":"2026-08-28T00:00:00Z",
                "ws_url":"wss://madeonsol.com/ws/v1/stream","usage":"connect"}"#,
        )
        .unwrap();
        assert_eq!(old.expires_at.as_deref(), Some("2026-08-28T00:00:00Z"));
        assert!(old.rotated.is_none());
        assert!(old.lifetime.is_none());
    }

    /// 2026-10-03 parity: verified holdings on /positions + summary.holdings.
    #[test]
    fn positions_deserialize_verified_holdings() {
        let r: crate::types::WalletPositionsResponse = serde_json::from_str(
            r#"{"chain":"robinhood","address":"0xabc","window_days":90,
                "summary":{"open_positions":1,"total_cost_basis_eth":1.0,"total_current_value_eth":2.0,
                  "total_unrealized_eth":1.0,"unpriced_positions":0,
                  "holdings":{"balance_source":"rhc_node_multicall3","checked_at":"2026-10-02T00:00:00Z",
                    "complete":true,"fifo_open_positions":1,"held":0,"partially_reduced":0,
                    "transferred_or_disposed":1,"external_inflow":0,"unverified":0,
                    "verified_value_eth":0.0,"unpriced_held":0,"cost_basis_held_eth":0.0,
                    "unrealized_known_eth":0.0,"cost_basis_not_held_eth":1.0}},
                "positions":[{"token_address":"0xt","token_symbol":null,"token_name":null,
                  "launchpad":null,"is_graduated":null,"token_amount":5.0,"cost_basis_eth":1.0,
                  "avg_entry_price_eth":0.2,"current_price_eth":0.4,"current_value_eth":2.0,
                  "unrealized_eth":1.0,"unrealized_pct":100.0,"current_mc_usd":null,
                  "liquidity_usd":null,"liquidity_basis":"measured","buys_in_position":1,
                  "realized_so_far_eth":0.0,"first_buy_at":null,"last_buy_at":null,
                  "fifo_unmatched_amount":5.0,"current_onchain_balance":0.0,
                  "holding_status":"TRANSFERRED_OR_DISPOSED","holding_unverified_reason":null,
                  "current_holding_value_eth":0.0}],"notes":{}}"#,
        )
        .unwrap();
        assert_eq!(r.summary.holdings.unwrap().transferred_or_disposed, 1);
        assert_eq!(
            r.positions[0].holding_status.as_deref(),
            Some("TRANSFERRED_OR_DISPOSED")
        );
    }

    /// Lock / unlock / early-buyer shapes parse from wire samples, including the
    /// all-null amount fields an unpriced token returns.
    #[test]
    fn token_lock_responses_parse() {
        let feed: types::TokenLocksResponse = serde_json::from_str(r#"{
            "chain":"robinhood",
            "locks":[{
                "lock_id":"0xabc:3","locker":"0x1111111111111111111111111111111111111111","locker_name":null,
                "provider":{"id":null,"name":null,"identity":"compatible","compatible_with":"pinklock","website_url":null,"lock_url":null},
                "explorer":{"locker_url":"https://explorer/x","creation_tx_url":null},
                "family":"pinklock","family_name":"PinkLock-compatible","locker_lock_id":"7",
                "kind":"lock","subject":"token","status":"active",
                "token_address":"0x2222222222222222222222222222222222222222","lp":null,
                "sender":"0x3333333333333333333333333333333333333333","recipient":null,"tx_sender":null,"name":null,
                "amount_raw":"1000000","amount":null,"amount_usd":null,"price_usd":null,"amount_pct_of_supply":null,"amount_unit":"token",
                "locked_raw":"1000000","locked":null,"locked_usd":null,"locked_pct_of_supply":null,
                "unlocked_raw":"0","unlocked":null,"withdrawn_raw":null,"withdrawn":null,
                "start_at":null,"cliff_at":null,"end_at":"2027-01-01T00:00:00Z",
                "seconds_until_end":7776000,"seconds_until_next_unlock":null,
                "cliff_amount_raw":null,"cliff_amount":null,"continuous":false,"perpetual":false,
                "next_unlock":{"at":"2027-01-01T00:00:00Z","kind":"final","amount_raw":"1000000","amount":null,"amount_usd":null},
                "cancelable":null,"cancelable_by_sender":null,"transferable":null,
                "created_at":"2026-10-01T00:00:00Z","created_at_estimated":false,
                "block_number":61824361,"block_time":"2026-10-01T00:00:00Z","tx_hash":"0xabc","log_index":3,"layout_verified":true
            }],
            "pagination":{"limit":50,"count":1,"has_more":false,"next_cursor":null,"next_since":"2026-10-01T00:00:00Z","next_before":"2026-10-01T00:00:00Z"},
            "stream":{"channel":"rhc:token_locks"},
            "coverage":{"families":["pinklock"],"withdrawals_tracked":false,"cancels_tracked":false,"lp_locks":"excluded","note":"n"},
            "meta":{"families":["pinklock"],"note":"n"}
        }"#).unwrap();
        assert_eq!(feed.locks.len(), 1);
        assert_eq!(feed.locks[0].provider.as_ref().unwrap().identity, "compatible");
        assert!(feed.locks[0].withdrawn.is_none());
        assert!(!feed.pagination.has_more);

        let unlocks: types::TokenUnlocksResponse = serde_json::from_str(r#"{
            "chain":"robinhood",
            "window":{"within":"7d","from":"2026-10-03T00:00:00Z","to":"2026-10-10T00:00:00Z"},
            "unlocks":[{
                "unlock_at":"2026-10-05T00:00:00Z","in_seconds":172800,"event":"cliff",
                "amount_raw":"5","amount":null,"amount_usd":null,"amount_pct_of_supply":null,
                "window_amount_raw":"5","window_amount":null,"window_amount_usd":null,"window_amount_pct_of_supply":null,
                "token_address":"0x2222222222222222222222222222222222222222",
                "token":{"symbol":"X","name":null,"decimals":18,"price_usd":null,"market_cap_usd":null},
                "lock":{"lock_id":"0xabc:3","locker":"0x1","locker_name":null,"family":"sablier","kind":"vesting","name":null,
                        "sender":"0x3","recipient":null,"amount_raw":"5","amount":null,"amount_usd":null,
                        "locked_raw":"5","locked":null,"locked_usd":null,"cliff_at":null,"end_at":null,
                        "cancelable_by_sender":true,"tx_hash":"0xabc"}
            }],
            "pagination":{"limit":50,"count":1,"total_in_window":1,"has_more":false,"candidates_capped":false},
            "coverage":{"families":[],"withdrawals_tracked":false,"cancels_tracked":false,"lp_locks":"n","note":"n"},
            "meta":{"families":[],"note":"n","source":"s"}
        }"#).unwrap();
        assert_eq!(unlocks.unlocks[0].event, "cliff");
        assert_eq!(unlocks.window.within, "7d");

        let summary: types::TokenLockSummaryResponse = serde_json::from_str(r#"{
            "chain":"robinhood","token_address":"0x2222222222222222222222222222222222222222",
            "token":{"symbol":null,"name":null,"decimals":null,"price_usd":null,"supply":null,"market_cap_usd":null,"liquidity_usd":null,"facts_resolved":false},
            "summary":{"lock_count":0,"complete":true,"rows_considered":0,"token_lock_count":0,"lp_lock_count":0,"lp_lock_active_count":0,"active_count":0,
                "by_family":{},"by_kind":{},"distinct_lockers":0,"distinct_locker_contracts":0,
                "locked_raw":"0","locked":null,"locked_usd":null,"locked_pct_of_supply":null,
                "deposited_raw":"0","deposited":null,"deposited_usd":null,
                "unlocking_7d_raw":"0","unlocking_7d":null,"unlocking_7d_usd":null,"unlocking_7d_pct_of_supply":null,
                "unlocking_30d_raw":"0","unlocking_30d":null,"unlocking_30d_usd":null,"unlocking_30d_pct_of_supply":null,
                "next_unlock":null,"active_cancelable_by_sender":0},
            "locks":[],
            "coverage":{"families":[],"withdrawals_tracked":false,"cancels_tracked":false,"lp_locks":"n","note":"n"},
            "meta":{"families":[],"note":"n","source":"s"}
        }"#).unwrap();
        assert_eq!(summary.summary.lock_count, 0);

        // Unranked token: empty list, summary null, note present.
        let empty: types::EarlyBuyersResponse = serde_json::from_str(r#"{
            "chain":"robinhood","token_address":"0x2222222222222222222222222222222222222222",
            "early_buyers":[],"count":0,"computed_at":null,"holdings_verified":null,"summary":null,"note":"not yet ranked"
        }"#).unwrap();
        assert!(empty.summary.is_none());

        let ranked: types::EarlyBuyersResponse = serde_json::from_str(r#"{
            "chain":"robinhood","token_address":"0x2222222222222222222222222222222222222222",
            "early_buyers":[{"rank":1,"wallet":"0x4","first_buy_at":"2026-10-01T00:00:00Z","first_buy_block":1,
                "still_holding":null,"balance":null,"position":"unknown","bought_eth":null,"sold_eth":null,
                "realized_eth":null,"trades":null,"avg_entry_mc_usd":null}],
            "count":1,"computed_at":"2026-10-02T00:00:00Z","holdings_verified":true,
            "summary":{"ranked":1,"with_holding_data":0,"still_holding":0,"exited":0,"closed_positions":0,"realized_eth_closed_only":0},
            "source":{"method":"first_buy_rank_from_trade_ingest","note":"n"}
        }"#).unwrap();
        assert_eq!(ranked.early_buyers[0].position, "unknown");
        assert_eq!(ranked.holdings_verified, Some(true));

        // Params serialize only what is set.
        let q = serde_json::to_value(&types::TokenLocksParams { cursor: Some("c".into()), ..Default::default() }).unwrap();
        assert_eq!(q, serde_json::json!({"cursor":"c"}));
    }
}
