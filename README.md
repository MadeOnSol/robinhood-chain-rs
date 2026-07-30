# robinhood-chain

[![Crates.io](https://img.shields.io/crates/v/robinhood-chain?style=flat-square)](https://crates.io/crates/robinhood-chain)
[![docs.rs](https://img.shields.io/docsrs/robinhood-chain?style=flat-square)](https://docs.rs/robinhood-chain)
[![Crates.io downloads](https://img.shields.io/crates/d/robinhood-chain?style=flat-square)](https://crates.io/crates/robinhood-chain)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)

> ⭐ **[Star on GitHub](https://github.com/madeonsol/robinhood-chain-rs)** · 📂 **[Examples](./examples/)** · 📚 **[docs.rs](https://docs.rs/robinhood-chain)** · 🌐 **[Robinhood Chain](https://madeonsol.com/robinhood)**

**Robinhood Chain SDK for Rust — EVM-native trading intelligence, chain id 4663.**

Typed, async, `tokio`-based, `rustls`-only client for the [MadeOnSol](https://madeonsol.com) Robinhood Chain API: live KOL trades, token discovery, launch-bundle detection, deployer reputation, smart-money wallet ranking, 1-minute OHLC candles, and the DEX trade tape — all served from our self-hosted Robinhood Chain node.

Robinhood Chain is an **Arbitrum Orbit L2**, so every field is EVM-native — `token_address` (lowercase `0x…`), `eth_amount`, `tx_hash`, `block_number`, `net_flow_eth`. No Solana field names.

> Robinhood Chain coverage is **bundled into every MadeOnSol tier at no extra cost** — same `msk_` API key, same base URL (`https://madeonsol.com/api/v1`). Get a free key at **<https://madeonsol.com/pricing>**.

## Install

```toml
[dependencies]
robinhood-chain = "0.2"
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
| `client.kol` | KOL feed, activity leaderboard, consensus hot-tokens, coordination, first-touches, single-KOL profile |
| `client.trades` | The Robinhood Chain DEX trade tape (Uniswap v2/v3/v4) |
| `client.tokens` | Token discovery, per-token snapshot, OHLC candles, KOL-consensus, buyer-quality, launch bundle, batch reads |
| `client.deployer_hunter` | Deployer reputation: leaderboard, profile, trajectory, launch history, best-tokens, chain-wide stats, alerts, recent graduations |
| `client.alpha_wallets` | Smart-money wallet ranking |
| `client.stream` | WebSocket streaming token issuance + `rhc:kol_trades` / `rhc:trades` channels |

## Endpoint → method map (all 25 routes)

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
| 14 | `POST /rhc/token/batch` | `client.tokens.batch(&addresses)` | BASIC+ |
| 15 | `POST /rhc/tokens/batch/buyer-quality` | `client.tokens.batch_buyer_quality(&addresses)` | BASIC+ |
| 16 | `GET /rhc/deployer-hunter/leaderboard` | `client.deployer_hunter.leaderboard(&params)` | BASIC+ |
| 17 | `GET /rhc/deployer-hunter/{address}` | `client.deployer_hunter.profile(addr)` | BASIC+ |
| 18 | `GET /rhc/deployer-hunter/{address}/trajectory` | `client.deployer_hunter.trajectory(addr)` | BASIC+ |
| 19 | `GET /rhc/deployer-hunter/{address}/tokens` | `client.deployer_hunter.tokens(addr, &params)` | BASIC+ |
| 20 | `GET /rhc/deployer-hunter/{address}/history` | `client.deployer_hunter.history(addr, &params)` | PRO+ |
| 21 | `GET /rhc/deployer-hunter/best-tokens` | `client.deployer_hunter.best_tokens(&params)` | BASIC+ |
| 22 | `GET /rhc/deployer-hunter/recent-bonds` | `client.deployer_hunter.recent_bonds(&params)` | BASIC+ |
| 23 | `GET /rhc/deployer-hunter/stats` | `client.deployer_hunter.stats()` | BASIC+ |
| 24 | `GET /rhc/deployer-hunter/alerts` | `client.deployer_hunter.alerts(&params)` | BASIC+ |
| 25 | `GET /rhc/alpha-wallets` | `client.alpha_wallets.list(&params)` | PRO+ |

`BASIC+` = any valid key (including the free tier). `PRO+` = Pro or Ultra. Some
BASIC+ endpoints return richer field-gated payloads on higher tiers (e.g. the
launch-`bundle` cohort: BASIC gets the scalar signal, PRO the top-10 wallets,
ULTRA the full cohort with alpha-wallet identity).

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

## Streaming

Two WebSocket channels carry Robinhood Chain events live (same
`wss://madeonsol.com/ws/v1/stream` protocol as the Solana stream client):

| Channel constant | Value | Payload |
|---|---|---|
| `stream::RHC_KOL_TRADES` | `rhc:kol_trades` | Every tracked-KOL buy/sell on chain 4663 |
| `stream::RHC_TRADES` | `rhc:trades` | The full RHC DEX swap firehose (PRO+) |

```rust
let ws = client.stream.get_token().await?; // POST /stream/token (PRO+)
// Connect to `ws.ws_url` with `?token=<ws.token>` appended, then subscribe to
// robinhood_chain::api::stream::RHC_KOL_TRADES / RHC_TRADES.
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
