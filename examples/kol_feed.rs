//! Print the latest KOL trades on Robinhood Chain (chain id 4663).
//!
//! Run with:
//! ```sh
//! MADEONSOL_API_KEY=msk_... cargo run --example kol_feed
//! ```
//!
//! Robinhood Chain coverage is bundled into every tier — get a free key at
//! https://madeonsol.com/developer.

use robinhood_chain::{
    types::{KolFeedParams, TradeAction},
    RobinhoodChain,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("MADEONSOL_API_KEY")
        .expect("set MADEONSOL_API_KEY — get a free one at https://madeonsol.com/developer");

    let client = RobinhoodChain::new(api_key)?;

    let feed = client
        .kol
        .feed(&KolFeedParams {
            limit: Some(10),
            action: Some(TradeAction::Buy),
            ..Default::default()
        })
        .await?;

    println!("Latest {} KOL buys on {}:\n", feed.trades.len(), feed.chain);
    for t in feed.trades {
        println!(
            "  {:>20}  bought  {:<10}  for {:>8.4} ETH",
            t.kol_name.unwrap_or_else(|| t.evm_address[..8.min(t.evm_address.len())].to_string()),
            t.token_symbol.unwrap_or_else(|| "?".to_string()),
            t.eth_amount.unwrap_or(0.0),
        );
    }

    Ok(())
}
