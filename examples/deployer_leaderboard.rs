//! Print the top Robinhood Chain deployers by graduation rate.
//!
//! Run with:
//! ```sh
//! MADEONSOL_API_KEY=msk_... cargo run --example deployer_leaderboard
//! ```
//!
//! On RHC most launchpads are direct-to-DEX, so "graduation" is a $40K+ peak-MC
//! milestone and a "runner" reached $100K+. Get a free key (RHC bundled into
//! every tier) at https://madeonsol.com/developer.

use robinhood_chain::{
    types::{DeployerLeaderboardParams, DeployerLeaderboardSort, DeployerTier},
    RobinhoodChain,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("MADEONSOL_API_KEY")
        .expect("set MADEONSOL_API_KEY — get a free one at https://madeonsol.com/developer");

    let client = RobinhoodChain::new(api_key)?;

    let board = client
        .deployer_hunter
        .leaderboard(&DeployerLeaderboardParams {
            sort: Some(DeployerLeaderboardSort::GraduationRate),
            tier: Some(DeployerTier::Elite),
            limit: Some(10),
            ..Default::default()
        })
        .await?;

    println!("Top {} elite deployers on {}:\n", board.deployers.len(), board.chain);
    for d in board.deployers {
        println!(
            "  {}…  {:>3} deployed  {:>5.1}% grad  {:>5.1}% runner",
            &d.deployer_address[..10.min(d.deployer_address.len())],
            d.tokens_deployed,
            d.graduation_rate * 100.0,
            d.runner_rate * 100.0,
        );
    }

    Ok(())
}
