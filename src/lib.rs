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
//! same `msk_` key, same base URL. Get a free key at <https://madeonsol.com/developer>.
//! Paid tiers (PRO / ULTRA) unlock the DEX trade tape, token discovery, candles,
//! KOL-consensus, alpha-wallet ranking, and WebSocket streaming — see
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
//! - [`RobinhoodChain::kol`] — KOL feed, leaderboard, consensus hot-tokens, single-KOL profile
//! - [`RobinhoodChain::trades`] — the DEX trade tape (PRO+)
//! - [`RobinhoodChain::tokens`] — token discovery, per-token snapshot, candles, KOL-consensus, buyer-quality, bundle
//! - [`RobinhoodChain::deployer_hunter`] — deployer reputation leaderboard + single-deployer profile
//! - [`RobinhoodChain::alpha_wallets`] — smart-money wallet ranking (PRO+)
//! - [`RobinhoodChain::stream`] — WebSocket streaming token issuance + `rhc:kol_trades` / `rhc:trades` channels
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
    alpha_wallets::AlphaWallets, deployer_hunter::DeployerHunter, kol::Kol, stream::Stream,
    tokens::Tokens, trades::Trades,
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
    /// KOL trade intelligence: feed, leaderboard, consensus hot-tokens, profile.
    pub kol: Kol,
    /// The Robinhood Chain DEX trade tape (PRO+).
    pub trades: Trades,
    /// Token intelligence: discovery, snapshot, candles, KOL-consensus, buyer-quality, bundle.
    pub tokens: Tokens,
    /// Deployer reputation leaderboard + single-deployer profile.
    pub deployer_hunter: DeployerHunter,
    /// Smart-money wallet ranking (PRO+).
    pub alpha_wallets: AlphaWallets,
    /// WebSocket streaming token issuance + `rhc:kol_trades` / `rhc:trades` channels.
    pub stream: Stream,
}

impl RobinhoodChain {
    /// Construct a new client.
    ///
    /// `api_key` must start with `msk_`. Robinhood Chain coverage is bundled into
    /// every tier — get a free key at <https://madeonsol.com/developer>.
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
                 → Get a free key at https://madeonsol.com/developer (RHC bundled into every tier)\n\
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
}
