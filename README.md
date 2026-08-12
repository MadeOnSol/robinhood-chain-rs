# robinhood-chain

[![Crates.io](https://img.shields.io/crates/v/robinhood-chain?style=flat-square)](https://crates.io/crates/robinhood-chain)
[![docs.rs](https://img.shields.io/docsrs/robinhood-chain?style=flat-square)](https://docs.rs/robinhood-chain)
[![Crates.io downloads](https://img.shields.io/crates/d/robinhood-chain?style=flat-square)](https://crates.io/crates/robinhood-chain)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)

> ⭐ **[Star on GitHub](https://github.com/madeonsol/robinhood-chain-rs)** · 📂 **[Examples](./examples/)** · 📚 **[docs.rs](https://docs.rs/robinhood-chain)** · 🌐 **[Robinhood Chain](https://madeonsol.com/robinhood)**

**Robinhood Chain SDK for Rust — EVM-native trading intelligence, chain id 4663.**

Typed, async, `tokio`-based, `rustls`-only client for the [MadeOnSol](https://madeonsol.com) Robinhood Chain API: live KOL trades, token discovery, launch-bundle detection, deployer reputation, smart-money wallet ranking, 1-minute OHLC candles, the DEX trade tape, and four push **rule engines** (copy-trade, price alerts, KOL coordination, KOL first-touches) — all served from our self-hosted Robinhood Chain node.

Robinhood Chain is an **Arbitrum Orbit L2**, so every field is EVM-native — `token_address` (lowercase `0x…`), `eth_amount`, `tx_hash`, `block_number`, `net_flow_eth`. No Solana field names.

> Robinhood Chain coverage is **bundled into every MadeOnSol tier at no extra cost** — same `msk_` API key, same base URL (`https://madeonsol.com/api/v1`). Get a free key at **<https://madeonsol.com/pricing>**.

## Install

```toml
[dependencies]
robinhood-chain = "0.5"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Requires Rust 1.75+. Uses `reqwest` with `rustls-tls` (no OpenSSL dependency).

## Quick start

```rust
use robinhood_chain::{RobinhoodChain, types::{KolFeedParams, TradeAction}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Free key — RHC is bundled into every tier. https://madeonsol.com/pricing
    let client = RobinhoodChain::new(std::env::var("MADEONSOL_API_KEY")?)?;

    let feed = client.kol.feed(&KolFeedParams {
        limit: Some(10),
        action: Some(TradeAction::Buy),
        ..Default::default()
    }).await?;

    for t in feed.trades {
        println!("{:?} bought {:?} for {} ETH (ran {:?}×)",
            t.kol_name, t.token_symbol,
            t.eth_amount.unwrap_or(0.0), t.mc_multiple_since_trade);
    }
    Ok(())
}
```

Run the bundled examples:

```sh
export MADEONSOL_API_KEY=msk_...
cargo run --example kol_feed
cargo run --example deployer_leaderboard
```

## Namespaces

The `RobinhoodChain` client exposes namespaced sub-clients:

| Namespace | Purpose |
|---|---|
| `client.kol` | KOL feed, activity leaderboard, consensus hot-tokens, coordination, first-touches, single-KOL profile — plus the coordination-alert (PRO+) and first-touch-subscription (ULTRA+) rule engines |
| `client.trades` | The Robinhood Chain DEX trade tape (Uniswap v2/v3/v4) |
| `client.tokens` | Token discovery, per-token snapshot, OHLC candles, KOL-consensus, buyer-quality, launch bundle, top-traders, flow, peak-history, risk, holders, batch reads |
| `client.deployer_hunter` | Deployer reputation: leaderboard, profile, trajectory, launch history, best-tokens, chain-wide stats, alerts, recent graduations |
| `client.alpha_wallets` | Smart-money wallet ranking |
| `client.copytrade` | Copy-trade rule engine: rules + fired-signal history (PRO+) |
| `client.price_alerts` | Price-alert rule engine: alerts + dip/recovery events (PRO+) |
| `client.stream` | WebSocket streaming token issuance + the six `rhc:*` channels (see [Streaming](#streaming)) |

## Endpoint → method map (all 52 operations, 40 paths)

> The table previously claimed "all 25 routes" — it had silently omitted the five
> token-intel endpoints added in 0.3.0 (`top-traders`, `flow`, `peak-history`,
> `risk`, `holders`). Both the count and those rows are corrected here, and the
> CI route-parity guard now pins the full set.

| # | Endpoint | Method | Tier |
|---|---|---|---|
| 1 | `GET /rhc/kol/feed` | `client.kol.feed(&params)` | BASIC+ |
| 2 | `GET /rhc/kol/leaderboard` | `client.kol.leaderboard(&params)` | BASIC+ |
| 3 | `GET /rhc/kol/hot-tokens` | `client.kol.hot_tokens(&params)` | BASIC+ |
| 4 | `GET /rhc/kol/coordination` | `client.kol.coordination(&params)` | BASIC+ |
| 5 | `GET /rhc/kol/first-touches` | `client.kol.first_touches(&params)` | BASIC+ |
| 6 | `GET /rhc/kol/{wallet}` | `client.kol.wallet(addr)` | BASIC+ |
| 7 | `GET /rhc/trades` | `client.trades.list(&params)` | PRO+ |
| 8 | `GET /rhc/tokens` | `client.tokens.list(&params)` | PRO+ |
| 9 | `GET /rhc/tokens/{address}` | `client.tokens.get(addr)` | BASIC+ |
| 10 | `GET /rhc/tokens/{address}/candles` | `client.tokens.candles(addr, &params)` | PRO+ |
| 11 | `GET /rhc/tokens/{address}/kol-consensus` | `client.tokens.kol_consensus(addr)` | PRO+ |
| 12 | `GET /rhc/tokens/{address}/buyer-quality` | `client.tokens.buyer_quality(addr)` | BASIC+ |
| 13 | `GET /rhc/tokens/{address}/bundle` | `client.tokens.bundle(addr)` | BASIC+ |
| 14 | `GET /rhc/tokens/{address}/top-traders` | `client.tokens.top_traders(addr, &params)` | PRO+ |
| 15 | `GET /rhc/tokens/{address}/flow` | `client.tokens.flow(addr, &params)` | PRO+ |
| 16 | `GET /rhc/tokens/{address}/peak-history` | `client.tokens.peak_history(addr, &params)` | PRO+ |
| 17 | `GET /rhc/tokens/{address}/risk` | `client.tokens.risk(addr)` | PRO+ |
| 18 | `GET /rhc/tokens/{address}/holders` | `client.tokens.holders(addr, &params)` | PRO+ |
| 19 | `POST /rhc/token/batch` | `client.tokens.batch(&addresses)` | BASIC+ |
| 20 | `POST /rhc/tokens/batch/buyer-quality` | `client.tokens.batch_buyer_quality(&addresses)` | BASIC+ |
| 21 | `GET /rhc/deployer-hunter/leaderboard` | `client.deployer_hunter.leaderboard(&params)` | BASIC+ |
| 22 | `GET /rhc/deployer-hunter/{address}` | `client.deployer_hunter.profile(addr)` | BASIC+ |
| 23 | `GET /rhc/deployer-hunter/{address}/trajectory` | `client.deployer_hunter.trajectory(addr)` | BASIC+ |
| 24 | `GET /rhc/deployer-hunter/{address}/tokens` | `client.deployer_hunter.tokens(addr, &params)` | BASIC+ |
| 25 | `GET /rhc/deployer-hunter/{address}/history` | `client.deployer_hunter.history(addr, &params)` | PRO+ |
| 26 | `GET /rhc/deployer-hunter/best-tokens` | `client.deployer_hunter.best_tokens(&params)` | BASIC+ |
| 27 | `GET /rhc/deployer-hunter/recent-bonds` | `client.deployer_hunter.recent_bonds(&params)` | BASIC+ |
| 28 | `GET /rhc/deployer-hunter/stats` | `client.deployer_hunter.stats()` | BASIC+ |
| 29 | `GET /rhc/deployer-hunter/alerts` | `client.deployer_hunter.alerts(&params)` | BASIC+ |
| 30 | `GET /rhc/alpha-wallets` | `client.alpha_wallets.list(&params)` | PRO+ |
| 31 | `GET /rhc/copytrade/subscriptions` | `client.copytrade.list()` | PRO+ |
| 32 | `POST /rhc/copytrade/subscriptions` | `client.copytrade.create(&params)` | PRO+ |
| 33 | `GET /rhc/copytrade/subscriptions/{id}` | `client.copytrade.get(id)` | PRO+ |
| 34 | `PATCH /rhc/copytrade/subscriptions/{id}` | `client.copytrade.update(id, &params)` | PRO+ |
| 35 | `DELETE /rhc/copytrade/subscriptions/{id}` | `client.copytrade.delete(id)` | PRO+ |
| 36 | `GET /rhc/copytrade/signals` | `client.copytrade.signals(&params)` | PRO+ |
| 37 | `GET /rhc/price-alerts` | `client.price_alerts.list()` | PRO+ |
| 38 | `POST /rhc/price-alerts` | `client.price_alerts.create(&params)` | PRO+ |
| 39 | `GET /rhc/price-alerts/{id}` | `client.price_alerts.get(id)` | PRO+ |
| 40 | `PATCH /rhc/price-alerts/{id}` | `client.price_alerts.update(id, &params)` | PRO+ |
| 41 | `DELETE /rhc/price-alerts/{id}` | `client.price_alerts.delete(id)` | PRO+ |
| 42 | `GET /rhc/price-alerts/events` | `client.price_alerts.events(&params)` | PRO+ |
| 43 | `GET /rhc/kol/coordination/alerts` | `client.kol.coordination_alerts_list()` | PRO+ |
| 44 | `POST /rhc/kol/coordination/alerts` | `client.kol.coordination_alerts_create(&params)` | PRO+ |
| 45 | `GET /rhc/kol/coordination/alerts/{id}` | `client.kol.coordination_alerts_get(uuid)` | PRO+ |
| 46 | `PATCH /rhc/kol/coordination/alerts/{id}` | `client.kol.coordination_alerts_update(uuid, &params)` | PRO+ |
| 47 | `DELETE /rhc/kol/coordination/alerts/{id}` | `client.kol.coordination_alerts_delete(uuid)` | PRO+ |
| 48 | `GET /rhc/kol/first-touches/subscriptions` | `client.kol.first_touch_subscriptions_list()` | ULTRA+ |
| 49 | `POST /rhc/kol/first-touches/subscriptions` | `client.kol.first_touch_subscriptions_create(&params)` | ULTRA+ |
| 50 | `GET /rhc/kol/first-touches/subscriptions/{id}` | `client.kol.first_touch_subscriptions_get(uuid)` | ULTRA+ |
| 51 | `PATCH /rhc/kol/first-touches/subscriptions/{id}` | `client.kol.first_touch_subscriptions_update(uuid, &params)` | ULTRA+ |
| 52 | `DELETE /rhc/kol/first-touches/subscriptions/{id}` | `client.kol.first_touch_subscriptions_delete(uuid)` | ULTRA+ |

`BASIC+` = any valid key (including the free tier). `PRO+` = Pro or Ultra.
`ULTRA+` = Ultra or Business. Some BASIC+ endpoints return richer field-gated
payloads on higher tiers (e.g. the launch-`bundle` cohort: BASIC gets the scalar
signal, PRO the top-10 wallets, ULTRA the full cohort with alpha-wallet
identity).

Copy-trade and price-alert ids are `i64`; coordination-alert and first-touch
subscription ids are UUID `&str`.

## What the data is

- **KOL feed / leaderboard / hot-tokens** — every buy/sell from tracked Solana KOLs' verified EVM wallets on Robinhood Chain, attributed to the effective trading account (`tx.from`, or the ERC-4337 userOp sender when the trade was bundled). The KOL→EVM mapping is recovered by tracing each KOL's Solana→EVM bridge deposits (deBridge / Relay / Mayan / Wormhole) — a dataset unique to MadeOnSol. Hot-tokens surfaces tokens bought by 2+ distinct KOLs (a consensus signal).
- **KOL coordination + first-touches** — `coordination` goes a level deeper than hot-tokens: the cohort *composition* behind each consensus token (per-KOL buy/sell legs, `accumulating` vs `distributing`, `exited_count`, `time_to_consensus_sec`). `first_touches` is the earliest KOL entry per token — the discovery signal, with the MC at entry vs current/peak so you can score how the call aged.
- **Trade tape** (`client.trades`) — every Uniswap v2/v3/v4 swap on chain 4663, ~sub-second from execution, carrying the effective trading account (`trader_eoa` — `tx.from`, or the ERC-4337 userOp sender when bundled; never the router or the bundler), gas/ordering for MEV analysis, and KOL/deployer flags.
- **Token discovery + snapshot** — live-priced tokens with MC, liquidity, peak MC + drawdown, launchpad (pons / flap / clanker / hood.fun / noxa / virtuals), and deployer reputation tier.
- **Batch reads** — `tokens.batch()` resolves up to **50** tokens in one set-based call (three server-side queries regardless of batch size), echoing every requested address back so positions line up (`found: false` for unknowns). `tokens.batch_buyer_quality()` caps at **20**, deliberately: buyer-quality is a per-token cohort computation, not one set-based query, and a per-token failure degrades to an `error` entry instead of failing the batch.
- **Buyer-quality + bundle** — a 0–100 quality read on a token's first-20 buyer cohort, and same-block launch-bundle detection with current-held %. (No `atomic_tx` kind — RHC is an L2 with no atomic multi-signer tx, so a detected bundle is `same_block`.)
- **Deployer reputation** — 40k+ deployers with tier, trajectory (streaks, rolling 10-launch success rate, `improving`/`declining`/`stable`), full paginated launch history, and chain-wide stats. Most RHC launchpads are direct-to-DEX (no bonding curve), so "graduation" is a $40K+ peak-MC milestone and a "runner" reached $100K+.
- **Deployer alerts** — live signals when a tracked deployer ships a new token or one of their tokens graduates.
- **Alpha wallets** — the reverse of KOL discovery: RHC trader wallets ranked by realized on-chain performance (`net_eth`, `win_rate`, `memecoin_share`, `likely_bot`).
- **Token intel** — per-token `top_traders` (realized flow, not PnL — a holder ranks last), `flow` by trader cohort (`net_eth = sell − buy`, so positive means distribution), `peak_history` (two peaks, because recorded and observed disagree), EVM-native `risk` (sellability simulated at the chain head, never cached), and exact `holders` folded from `Transfer` logs.
- **Rule engines** — copy-trade, price alerts, KOL coordination and KOL first-touches, each pushing to a webhook or WebSocket with a queryable fire history. See [Rule engines](#rule-engines-push).

### Two things to know about deployer tiers

**Tiers ride the $100K runner rate, not the $40K graduation rate.** `elite` and `good` are earned on `runner_rate` and require 24h of deployer history — the $40K bar proved farmable by operators mass-relaunching one ticker across rotating wallets. `graduation_rate` is still returned and still means the $40K bar, but it no longer determines the tier; `spammer` is the one exception that still keys off it. `client.deployer_hunter.stats()` returns the live thresholds in `tier_rules`, so you never have to guess what `elite` currently means.

**Alerts filter for tradability by default, and resolve the tier at read time.** Alerts whose token has `liquidity_usd` below $100 — or unknown liquidity, which on RHC usually means a drained pool — are dropped: a $45K-MC alert on a token with $68 of liquidity is not a signal. Pass `include_untradeable: Some(true)` for the raw tape; the active setting comes back as `tradability_filter`. Each alert's `tier` is the deployer's *current* tier, with the snapshot taken when the alert fired alongside as `tier_at_alert` and `tier_is_stale` set when they disagree.

```rust
use robinhood_chain::types::{DeployerAlertsParams, DeployerTier};

let alerts = client.deployer_hunter.alerts(&DeployerAlertsParams {
    deployer_tier: Some(DeployerTier::Elite),
    limit: Some(25),
    ..Default::default()
}).await?;

for a in alerts.alerts {
    println!("{:?} {:?} — liquidity ${:?} (stale tier: {})",
        a.alert_type, a.token_symbol, a.liquidity_usd, a.tier_is_stale);
}
```

## Rule engines (push)

Four engines turn the read endpoints into push. Each rule delivers by webhook
(HMAC-SHA256 over `<timestamp>.<body>` in `X-MadeOnSol-Signature`), WebSocket, or
both, and each keeps a queryable fire history so you can catch up after a missed
delivery.

| Engine | Namespace | Fires when | History | Tier |
|---|---|---|---|---|
| Copy-trade | `client.copytrade` | a wallet you follow trades on RHC | `signals()`, 7 days | PRO+ |
| Price alerts | `client.price_alerts` | a token drops (and optionally bounces back) | `events()`, 30 days | PRO+ |
| KOL coordination | `client.kol.coordination_alerts_*` | N+ tracked KOLs buy the same token in a window | via WS / webhook | PRO+ |
| KOL first touches | `client.kol.first_touch_subscriptions_*` | a token gets its FIRST tracked-KOL buy | via WS / webhook | ULTRA+ |

**Quotas are per chain.** A full set of Solana rules consumes no Robinhood Chain
capacity, and vice versa.

**Three RHC-specific differences before you port Solana code:**

- **Price alerts are ~15s POLLED, not sub-second.** The RHC price writer emits no
  `pg_notify`, so alerts are polled off `rhc_token_prices`; effective latency is
  that interval plus the token's own price-update cadence. Every create response
  says so in its `evaluation` block.
- **Copy-trade has no market-cap band.** The RHC notify payload carries no market
  cap, so a band could only be a per-event DB lookup in the hot path of a
  ~3.3M-trades/day chain. Sizes are ETH, not SOL.
- **First-touch filters are strict and smaller.** RHC has no scout score, so
  `min_scout_tier` / `min_n_touches` are *absent* rather than silently matching
  nothing — `min_kol_winrate` and `strategy` are the quality gates, and unknown
  filter keys are rejected with a 400.

```rust
use robinhood_chain::types::*;

let created = client.copytrade.create(&CopyTradeCreateParams {
    name: Some("whale follow".into()),
    source_wallets: vec!["0xabc…".into()],
    sizing_mode: Some(CopyTradeSizingMode::Proportional),
    sizing_amount: 0.5,
    delivery_mode: Some(DeliveryMode::Websocket),
    ..Default::default()
}).await?;

// `webhook_secret` is minted once (whenever a webhook_url was supplied) — store it now.
println!("rule {} — {}", created.subscription.id, created.note);

// Catch up on anything the socket missed.
let fired = client.copytrade.signals(&CopyTradeSignalsParams {
    subscription_id: Some(created.subscription.id),
    limit: Some(100),
    ..Default::default()
}).await?;
println!("{} signals", fired.count);
```

### PATCH: omitting a field vs clearing it

On the update params, nullable fields are `Option<Option<T>>` so the two cases
stay distinguishable on the wire:

| Rust | JSON | Effect |
|---|---|---|
| `None` | key omitted | leave the stored value untouched |
| `Some(None)` | `"name": null` | clear the stored value |
| `Some(Some(v))` | `"name": "v"` | set it to `v` |

```rust
client.price_alerts.update(alert_id, &PriceAlertUpdateParams {
    name: Some(None),                 // clear the label
    is_active: Some(false),           // pause it
    ..Default::default()              // everything else untouched
}).await?;
```

Price alerts only accept `name`, `delivery_mode`, `webhook_url` and `is_active`
on PATCH — `token_address` / `drop_pct` / `recovery_pct` are immutable, because
retuning a threshold mid-flight would make the alert's recorded events
uninterpretable. First-touch `filters` is a whole-object **replace**, not a
merge, so "remove this filter" stays expressible.

## Streaming

Six WebSocket channels carry Robinhood Chain events live (same
`wss://madeonsol.com/ws/v1/stream` protocol as the Solana stream client):

| Channel constant | Value | Tier | Payload |
|---|---|---|---|
| `stream::RHC_KOL_TRADES` | `rhc:kol_trades` | PRO+ (connection gate) | Every tracked-KOL buy/sell on chain 4663 (`rhc:kol_trade` events) |
| `stream::RHC_DEX_TRADES` | `rhc:dex_trades` | **ULTRA+** | The full RHC DEX swap firehose, ~40-55 trades/s at tip (`rhc:dex_trade` events) |
| `stream::RHC_COPYTRADE_SIGNALS` | `rhc:copytrade:signals` | PRO+ | Your copy-trade rule fires (`rhc:copytrade:signal`, user-scoped) |
| `stream::RHC_PRICE_ALERT_EVENTS` | `rhc:price_alert:events` | PRO+ | Your price-alert fires (`rhc:price_alert:dip` / `rhc:price_alert:recovery`, user-scoped, ~15s polled) |
| `stream::RHC_KOL_COORDINATION` | `rhc:kol:coordination` | PRO+ | Your coordination-alert rule fires (`rhc:kol:coordination`, user-scoped) |
| `stream::RHC_KOL_FIRST_TOUCHES` | `rhc:kol:first_touches` | PRO+ | Every token's FIRST tracked-KOL buy (`rhc:kol:first_touch`, **broadcast** — ULTRA gates only the first-touch subscription CRUD, not this channel) |

> **Fixed in 0.5.0:** 0.4.0's `stream::RHC_TRADES` pointed at `rhc:trades`, a
> channel that never existed server-side — subscribing drew a
> `channels_rejected` warning and then silence — and its docs claimed PRO+
> where the real firehose gate is ULTRA+. The constant now carries the real
> name `rhc:dex_trades` (and is deprecated in favor of `RHC_DEX_TRADES`); the
> server additionally accepts `rhc:trades` as a deprecated alias for 0.4.0
> clients. The four rule-engine channel constants above are new in 0.5.0.

The server never fails a `subscribe` outright: channels your tier cannot
access are dropped and reported in a warning frame
(`{"type":"warning","code":"channels_rejected","rejected":[{"channel":"...","reason":"requires ULTRA"}],"valid_channels":[...],"ts":...}`)
followed by the normal `subscribed` ack. Watch for it — a rejected channel
otherwise looks like a healthy but silent subscription.

```rust
let ws = client.stream.get_token().await?; // POST /stream/token (PRO+)
// Connect to `ws.ws_url` with `?token=<ws.token>` appended, then subscribe to
// robinhood_chain::api::stream::RHC_KOL_TRADES / RHC_DEX_TRADES / the four
// rule-engine channel constants.
```

## Error handling

Every call returns `Result<T, robinhood_chain::Error>`. API errors carry the HTTP
status and the parsed body:

```rust
match client.tokens.get("0xnot_a_token").await {
    Ok(token) => println!("{:?}", token.symbol),
    Err(e) => {
        if let Some(404) = e.status() { println!("token not on Robinhood Chain"); }
        else { eprintln!("{e}"); }
    }
}
```

## Links

- Robinhood Chain overview — <https://madeonsol.com/robinhood>
- Pricing & tiers — <https://madeonsol.com/pricing>
- Full API reference — <https://madeonsol.com/api-docs>
- Get a free API key — <https://madeonsol.com/pricing>

## License

MIT © MadeOnSol
