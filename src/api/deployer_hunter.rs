use std::sync::Arc;

use crate::client::HttpCore;
use crate::error::Result;
use crate::types::*;

/// Deployer reputation on Robinhood Chain.
///
/// Most RHC launchpads are direct-to-DEX (no bonding curve), so "graduation" is
/// a market-cap milestone: `graduation_rate` = share of a deployer's tokens that
/// reached a $40K+ peak MC; `runner_rate` = share that reached $100K+.
#[derive(Debug, Clone)]
pub struct DeployerHunter {
    pub(crate) core: Arc<HttpCore>,
}

impl DeployerHunter {
    /// Deployer reputation leaderboard (`GET /rhc/deployer-hunter/leaderboard`, BASIC+).
    ///
    /// Deployers ranked by reputation, from a 5-min-refresh rollup over every
    /// launchpad token we've indexed (40k+ deployers). Page with `limit`/`offset`
    /// until `has_more` is false.
    pub async fn leaderboard(
        &self,
        params: &DeployerLeaderboardParams,
    ) -> Result<DeployerLeaderboardResponse> {
        self.core
            .get("/rhc/deployer-hunter/leaderboard", params)
            .await
    }

    /// Single deployer profile (`GET /rhc/deployer-hunter/{address}`, BASIC+).
    ///
    /// One deployer's full reputation row plus their 50 most recent tokens
    /// enriched with live MC and peak MC. Unknown wallets return 200 with
    /// `is_deployer: false` (not a 404) so clients can branch cheaply. `address`
    /// is an EVM wallet address (0x, 40 hex).
    pub async fn profile(&self, address: &str) -> Result<DeployerProfileResponse> {
        self.core
            .get(&format!("/rhc/deployer-hunter/{}", address), &())
            .await
    }
}
