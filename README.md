# robinhood-chain

[![Crates.io](https://img.shields.io/crates/v/robinhood-chain?style=flat-square)](https://crates.io/crates/robinhood-chain)
[![docs.rs](https://img.shields.io/docsrs/robinhood-chain?style=flat-square)](https://docs.rs/robinhood-chain)
[![Crates.io downloads](https://img.shields.io/crates/d/robinhood-chain?style=flat-square)](https://crates.io/crates/robinhood-chain)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)

> ⭐ **[Star on GitHub](https://github.com/madeonsol/robinhood-chain-rs)** · 📂 **[Examples](./examples/)** · 📚 **[docs.rs](https://docs.rs/robinhood-chain)** · 🌐 **[Robinhood Chain](https://madeonsol.com/robinhood)**

**Robinhood Chain SDK for Rust — EVM-native trading intelligence, chain id 4663.**

Typed, async, `tokio`-based, `rustls`-only client for the [MadeOnSol](https://madeonsol.com) Robinhood Chain API: live KOL trades, token discovery, launch-bundle detection, deployer reputation, smart-money wallet ranking, 1-minute OHLC candles, and the DEX trade tape — all served from our self-hosted Robinhood Chain node.

Robinhood Chain is an **Arbitrum Orbit L2**, so every field is EVM-native — `token_address` (lowercase `0x…`), `eth_amount`, `tx_hash`, `block_number`, `net_flow_eth`. No Solana field names.

> Robinhood Chain coverage is **bundled into every MadeOnSol tier at no extra cost** — same `msk_` API key, same base URL (`https://madeonsol.com/api/v1`). Get a free key at **<https://madeonsol.com/developer>**.

## Install

```toml
[dependencies]
robinhood-chain = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Requires Rust 1.75+. Uses `reqwest` with `rustls-tls` (no OpenSSL dependency).

## Quick start

```rust
use robinhood_chain::{RobinhoodChain, types::{KolFeedParams, TradeAction}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Free key — RHC is bundled into every tier. https://madeonsol.com/developer
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
| `client.kol` | KOL feed, activity leaderboard, consensus hot-tokens, single-KOL profile |
| `client.trades` | The Robinhood Chain DEX trade tape (Uniswap v2/v3/v4) |
| `client.tokens` | Token discovery, per-token snapshot, OHLC candles, KOL-consensus, buyer-quality, launch bundle |
| `client.deployer_hunter` | Deployer reputation leaderboard + single-deployer profile |
| `client.alpha_wallets` | Smart-money wallet ranking |
| `client.stream` | WebSocket streaming token issuance + `rhc:kol_trades` / `rhc:trades` channels |

## Endpoint → method map (all 14 routes)

| # | Endpoint | Method | Tier |
|---|---|---|---|
| 1 | `GET /rhc/kol/feed` | `client.kol.feed(&params)` | BASIC+ |
| 2 | `GET /rhc/kol/leaderboard` | `client.kol.leaderboard(&params)` | BASIC+ |
| 3 | `GET /rhc/kol/hot-tokens` | `client.kol.hot_tokens(&params)` | BASIC+ |
| 4 | `GET /rhc/kol/{wallet}` | `client.kol.wallet(addr)` | BASIC+ |
| 5 | `GET /rhc/trades` | `client.trades.list(&params)` | PRO+ |
| 6 | `GET /rhc/tokens` | `client.tokens.list(&params)` | PRO+ |
| 7 | `GET /rhc/tokens/{address}` | `client.tokens.get(addr)` | BASIC+ |
| 8 | `GET /rhc/tokens/{address}/candles` | `client.tokens.candles(addr, &params)` | PRO+ |
| 9 | `GET /rhc/tokens/{address}/kol-consensus` | `client.tokens.kol_consensus(addr)` | PRO+ |
| 10 | `GET /rhc/tokens/{address}/buyer-quality` | `client.tokens.buyer_quality(addr)` | BASIC+ |
| 11 | `GET /rhc/tokens/{address}/bundle` | `client.tokens.bundle(addr)` | BASIC+ |
| 12 | `GET /rhc/deployer-hunter/leaderboard` | `client.deployer_hunter.leaderboard(&params)` | BASIC+ |
| 13 | `GET /rhc/deployer-hunter/{address}` | `client.deployer_hunter.profile(addr)` | BASIC+ |
| 14 | `GET /rhc/alpha-wallets` | `client.alpha_wallets.list(&params)` | PRO+ |

`BASIC+` = any valid key (including the free tier). `PRO+` = Pro or Ultra. Some
BASIC+ endpoints return richer field-gated payloads on higher tiers (e.g. the
launch-`bundle` cohort: BASIC gets the scalar signal, PRO the top-10 wallets,
ULTRA the full cohort with alpha-wallet identity).

## What the data is

- **KOL feed / leaderboard / hot-tokens** — every buy/sell from tracked Solana KOLs' verified EVM wallets on Robinhood Chain, attributed via `tx.from`. The KOL→EVM mapping is recovered by tracing each KOL's Solana→EVM bridge deposits (deBridge / Relay / Mayan / Wormhole) — a dataset unique to MadeOnSol. Hot-tokens surfaces tokens bought by 2+ distinct KOLs (a consensus signal).
- **Trade tape** (`client.trades`) — every Uniswap v2/v3/v4 swap on chain 4663, ~sub-second from execution, carrying the real trader wallet (`trader_eoa`), gas/ordering for MEV analysis, and KOL/deployer flags.
- **Token discovery + snapshot** — live-priced tokens with MC, liquidity, peak MC + drawdown, launchpad (pons / flap / clanker / hood.fun / noxa / virtuals), and deployer reputation tier.
- **Buyer-quality + bundle** — a 0–100 quality read on a token's first-20 buyer cohort, and same-block launch-bundle detection with current-held %. (No `atomic_tx` kind — RHC is an L2 with no atomic multi-signer tx, so a detected bundle is `same_block`.)
- **Deployer reputation** — 40k+ deployers ranked by graduation rate. Most RHC launchpads are direct-to-DEX (no bonding curve), so "graduation" is a $40K+ peak-MC milestone and a "runner" reached $100K+.
- **Alpha wallets** — the reverse of KOL discovery: RHC trader wallets ranked by realized on-chain performance (`net_eth`, `win_rate`, `memecoin_share`, `likely_bot`).

## Streaming

Two WebSocket channels carry Robinhood Chain events live (same
`wss://madeonsol.com/stream` protocol as the Solana stream client):

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
- Get a free API key — <https://madeonsol.com/developer>

## License

MIT © MadeOnSol
