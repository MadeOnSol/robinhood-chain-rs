//! Request and response types for every Robinhood Chain endpoint.
//!
//! Robinhood Chain is an EVM chain (id 4663), so every wire field is EVM-native:
//! `token_address` (lowercase `0x…`), `eth_amount`, `tx_hash`, `block_number`,
//! `net_flow_eth`. Field names mirror the JSON wire format exactly so you can paste
//! API examples straight from the docs at <https://madeonsol.com/api-docs>.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ─── Shared enums ────────────────────────────────────────────────────────────

/// Trade side.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TradeAction {
    Buy,
    Sell,
}

impl TradeAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Buy => "buy",
            Self::Sell => "sell",
        }
    }
}

/// Rolling window for the KOL leaderboard.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum KolLeaderboardPeriod {
    #[serde(rename = "24h")]
    H24,
    #[serde(rename = "7d")]
    D7,
    #[serde(rename = "30d")]
    D30,
}

impl KolLeaderboardPeriod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::H24 => "24h",
            Self::D7 => "7d",
            Self::D30 => "30d",
        }
    }
}

/// Rolling consensus window for hot tokens.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HotTokensWindow {
    #[serde(rename = "5m")]
    M5,
    #[serde(rename = "15m")]
    M15,
    #[serde(rename = "1h")]
    H1,
    #[serde(rename = "6h")]
    H6,
    #[serde(rename = "24h")]
    H24,
}

impl HotTokensWindow {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::M5 => "5m",
            Self::M15 => "15m",
            Self::H1 => "1h",
            Self::H6 => "6h",
            Self::H24 => "24h",
        }
    }
}

/// Uniswap DEX version filter for the trade tape.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TradeDex {
    #[serde(rename = "uniswap-v2")]
    UniswapV2,
    #[serde(rename = "uniswap-v3")]
    UniswapV3,
    #[serde(rename = "uniswap-v4")]
    UniswapV4,
}

impl TradeDex {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UniswapV2 => "uniswap-v2",
            Self::UniswapV3 => "uniswap-v3",
            Self::UniswapV4 => "uniswap-v4",
        }
    }
}

/// Ordering for token discovery (all descending).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TokensSort {
    LastTrade,
    MarketCap,
    Liquidity,
    PeakMc,
}

impl TokensSort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LastTrade => "last_trade",
            Self::MarketCap => "market_cap",
            Self::Liquidity => "liquidity",
            Self::PeakMc => "peak_mc",
        }
    }
}

/// Deployer reputation tier (filter + response value).
///
/// ⚠️ Tier semantics (migrations 267 + 269): `Elite` and `Good` are earned on the
/// $100K `runner_rate` and require 24h of deployer history — the $40K
/// graduation bar proved farmable by operators mass-relaunching one ticker
/// across rotating wallets. `graduation_rate` is still returned and still means
/// the $40K bar, but it NO LONGER sets the tier; `Spammer` is the one exception
/// that still keys off it. Call
/// [`DeployerHunter::stats`](crate::api::deployer_hunter::DeployerHunter::stats)
/// for the live thresholds.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DeployerTier {
    Elite,
    Good,
    Neutral,
    Spammer,
}

impl DeployerTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Elite => "elite",
            Self::Good => "good",
            Self::Neutral => "neutral",
            Self::Spammer => "spammer",
        }
    }
}

/// Ordering for the deployer-hunter leaderboard (all descending, NULLs last).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeployerLeaderboardSort {
    GraduationRate,
    RunnerRate,
    TokensDeployed,
    BestPeakMcUsd,
    LastDeployAt,
}

impl DeployerLeaderboardSort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GraduationRate => "graduation_rate",
            Self::RunnerRate => "runner_rate",
            Self::TokensDeployed => "tokens_deployed",
            Self::BestPeakMcUsd => "best_peak_mc_usd",
            Self::LastDeployAt => "last_deploy_at",
        }
    }
}

/// Behavioural classification filter for alpha-wallet ranking.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AlphaClassification {
    All,
    Human,
    Bot,
    SmartMoney,
}

impl AlphaClassification {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Human => "human",
            Self::Bot => "bot",
            Self::SmartMoney => "smart_money",
        }
    }
}

/// Identity filter for alpha-wallet ranking.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AlphaIdentity {
    All,
    KnownKol,
    Unknown,
}

impl AlphaIdentity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::KnownKol => "known_kol",
            Self::Unknown => "unknown",
        }
    }
}

/// Ordering for alpha-wallet ranking.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AlphaSort {
    NetEth,
    WinRate,
    Trades,
    Tokens,
    BuyEth,
    MemecoinShare,
    LastTradeAt,
}

impl AlphaSort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NetEth => "net_eth",
            Self::WinRate => "win_rate",
            Self::Trades => "trades",
            Self::Tokens => "tokens",
            Self::BuyEth => "buy_eth",
            Self::MemecoinShare => "memecoin_share",
            Self::LastTradeAt => "last_trade_at",
        }
    }
}

/// Sort direction.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Desc,
    Asc,
}

impl SortOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Desc => "desc",
            Self::Asc => "asc",
        }
    }
}

/// Confidence band on a buyer-quality read.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    Low,
    Medium,
    High,
}

/// Directional signal on a buyer-quality read.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum QualitySignal {
    Positive,
    Neutral,
    Negative,
}

/// Launch-bundle kind. Robinhood Chain is an Arbitrum Orbit L2 with no atomic
/// multi-signer transaction, so there is no `atomic_tx` kind — a detected bundle
/// is `same_block`, else `none`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BundleKind {
    SameBlock,
    None,
}

/// Window for the reputable-deployer best-tokens ranking.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BestTokensPeriod {
    #[serde(rename = "24h")]
    H24,
    #[serde(rename = "7d")]
    D7,
    #[serde(rename = "30d")]
    D30,
    #[serde(rename = "all")]
    All,
}

impl BestTokensPeriod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::H24 => "24h",
            Self::D7 => "7d",
            Self::D30 => "30d",
            Self::All => "all",
        }
    }
}

/// Ordering for one deployer's paginated launch history.
///
/// `PeakMcUsd` is a **page-scoped** sort — peak MC lives in another table and is
/// applied after enrichment of the requested page only, so it is not a global
/// "top tokens by peak MC" ranking. Use
/// [`DeployerHunter::best_tokens`](crate::api::deployer_hunter::DeployerHunter::best_tokens)
/// for a real ranking.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeployerTokensSort {
    FirstSeenAt,
    PeakMcUsd,
}

impl DeployerTokensSort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FirstSeenAt => "first_seen_at",
            Self::PeakMcUsd => "peak_mc_usd",
        }
    }
}

/// Rolling window for KOL coordination.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CoordinationPeriod {
    #[serde(rename = "1h")]
    H1,
    #[serde(rename = "6h")]
    H6,
    #[serde(rename = "24h")]
    H24,
    #[serde(rename = "7d")]
    D7,
}

impl CoordinationPeriod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::H1 => "1h",
            Self::H6 => "6h",
            Self::H24 => "24h",
            Self::D7 => "7d",
        }
    }
}

/// Deployer-alert kind. Robinhood Chain has no `bonded`/`kol_buy` alerts — most
/// launchpads are direct-to-DEX and alerts carry no KOL join.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeployerAlertType {
    NewDeploy,
    Graduated,
}

impl DeployerAlertType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NewDeploy => "new_deploy",
            Self::Graduated => "graduated",
        }
    }
}

/// Deployer-alert priority. Robinhood Chain has no `low` priority.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DeployerAlertPriority {
    High,
    Medium,
}

impl DeployerAlertPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
        }
    }
}

/// Direction of a KOL cohort's in-window flow on a coordination row.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CoordinationSignal {
    Accumulating,
    Distributing,
}

/// Direction of a deployer's launch quality over time.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TrajectoryTrend {
    Improving,
    Declining,
    Stable,
}

// ─── KOL: feed ───────────────────────────────────────────────────────────────

/// Query parameters for [`Kol::feed`](crate::api::kol::Kol::feed).
#[derive(Debug, Clone, Default, Serialize)]
pub struct KolFeedParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor — return trades strictly older than this ISO 8601 timestamp.
    /// Pass `next_before` from the previous response to page backwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<TradeAction>,
    /// Filter to a single KOL by their EVM wallet address (0x, 40 hex).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kol: Option<String>,
    /// Minimum trade size in ETH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_eth: Option<f64>,
}

/// A single KOL trade on Robinhood Chain.
#[derive(Debug, Clone, Deserialize)]
pub struct RhcKolTrade {
    /// The KOL's Robinhood-Chain wallet (0x).
    pub evm_address: String,
    #[serde(default)]
    pub kol_name: Option<String>,
    #[serde(default)]
    pub kol_twitter: Option<String>,
    pub token_address: String,
    #[serde(default)]
    pub token_symbol: Option<String>,
    #[serde(default)]
    pub token_name: Option<String>,
    /// `pons` | `flap` | `clanker` | `hood.fun` | `virtuals` | null.
    #[serde(default)]
    pub launchpad: Option<String>,
    #[serde(default)]
    pub is_graduated: Option<bool>,
    /// Reputation tier of the token's deployer (`elite`/`good`/`neutral`/`spammer`).
    #[serde(default)]
    pub deployer_tier: Option<String>,
    /// Token age at request time (first-seen → now), in minutes.
    #[serde(default)]
    pub token_age_minutes: Option<i64>,
    pub action: TradeAction,
    /// Trade size in ETH.
    #[serde(default)]
    pub eth_amount: Option<f64>,
    #[serde(default)]
    pub token_amount: Option<f64>,
    #[serde(default)]
    pub price_usd_at_trade: Option<f64>,
    /// Token market cap when the KOL traded.
    #[serde(default)]
    pub market_cap_usd_at_trade: Option<f64>,
    /// Token market cap now.
    #[serde(default)]
    pub current_mc_usd: Option<f64>,
    /// All-time-high market cap observed since ingestion.
    #[serde(default)]
    pub peak_mc_usd: Option<f64>,
    #[serde(default)]
    pub liquidity_usd: Option<f64>,
    /// `current_mc_usd ÷ market_cap_usd_at_trade` — how far the token ran after the trade.
    #[serde(default)]
    pub mc_multiple_since_trade: Option<f64>,
    /// `uniswap-v2/v3/v4` or the launchpad name for curve trades.
    pub dex: String,
    #[serde(default)]
    pub pool: Option<String>,
    pub tx_hash: String,
    pub block_number: i64,
    pub traded_at: String,
}

/// Response of [`Kol::feed`](crate::api::kol::Kol::feed).
#[derive(Debug, Clone, Deserialize)]
pub struct KolFeedResponse {
    /// Always `"robinhood"`.
    pub chain: String,
    pub trades: Vec<RhcKolTrade>,
    pub count: u32,
    /// Age of the newest row, in seconds.
    #[serde(default)]
    pub data_age_seconds: Option<i64>,
    /// Cursor for the next page — pass as `before` to fetch older trades.
    #[serde(default)]
    pub next_before: Option<String>,
}

// ─── KOL: leaderboard ────────────────────────────────────────────────────────

/// Query parameters for [`Kol::leaderboard`](crate::api::kol::Kol::leaderboard).
#[derive(Debug, Clone, Default, Serialize)]
pub struct KolLeaderboardParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<KolLeaderboardPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// A single row of the KOL activity leaderboard.
#[derive(Debug, Clone, Deserialize)]
pub struct KolLeaderboardEntry {
    #[serde(default)]
    pub kol_name: Option<String>,
    #[serde(default)]
    pub kol_twitter: Option<String>,
    pub trades: i64,
    pub buys: i64,
    pub sells: i64,
    /// Total ETH bought in the window.
    pub buy_eth: f64,
    /// Total ETH sold in the window.
    pub sell_eth: f64,
    /// `buy_eth − sell_eth` (flow, not realized PnL).
    pub net_eth: f64,
    /// Distinct tokens traded in the window.
    pub tokens_traded: i64,
    pub last_trade_at: String,
}

/// Response of [`Kol::leaderboard`](crate::api::kol::Kol::leaderboard).
#[derive(Debug, Clone, Deserialize)]
pub struct KolLeaderboardResponse {
    pub chain: String,
    pub period: String,
    pub leaderboard: Vec<KolLeaderboardEntry>,
    pub count: u32,
}

// ─── KOL: hot tokens ─────────────────────────────────────────────────────────

/// Query parameters for [`Kol::hot_tokens`](crate::api::kol::Kol::hot_tokens).
#[derive(Debug, Clone, Default, Serialize)]
pub struct HotTokensParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<HotTokensWindow>,
}

/// A consensus token — bought by 2+ distinct KOLs in the window.
#[derive(Debug, Clone, Deserialize)]
pub struct HotToken {
    pub token_address: String,
    #[serde(default)]
    pub token_symbol: Option<String>,
    #[serde(default)]
    pub token_name: Option<String>,
    /// `noxa | flap | pons | hood.fun | clanker | null`.
    #[serde(default)]
    pub launchpad: Option<String>,
    #[serde(default)]
    pub is_graduated: Option<bool>,
    /// `elite | good | neutral | spammer | null`.
    #[serde(default)]
    pub deployer_tier: Option<String>,
    /// Distinct KOL buyers in the window (>= 2).
    pub kols_buying: i64,
    pub buys: i64,
    pub sells: i64,
    pub buy_eth: f64,
    /// `buy_eth − sell_eth`.
    pub net_eth: f64,
    /// Current market cap.
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    pub last_trade_at: String,
}

/// Response of [`Kol::hot_tokens`](crate::api::kol::Kol::hot_tokens).
#[derive(Debug, Clone, Deserialize)]
pub struct HotTokensResponse {
    pub chain: String,
    pub window: String,
    pub tokens: Vec<HotToken>,
    pub count: u32,
}

// ─── KOL: profile ────────────────────────────────────────────────────────────

/// Aggregate stats over a KOL's last 200 RHC trades.
#[derive(Debug, Clone, Deserialize)]
pub struct KolProfileStats {
    pub trades: i64,
    pub buys: i64,
    pub sells: i64,
    pub buy_eth: f64,
    pub sell_eth: f64,
    pub net_eth: f64,
    pub tokens_traded: i64,
    /// e.g. `"last 200 trades"`.
    pub window: String,
}

/// Response of [`Kol::wallet`](crate::api::kol::Kol::wallet).
#[derive(Debug, Clone, Deserialize)]
pub struct KolProfileResponse {
    pub chain: String,
    pub evm_address: String,
    #[serde(default)]
    pub kol_name: Option<String>,
    #[serde(default)]
    pub kol_twitter: Option<String>,
    pub stats: KolProfileStats,
    /// 50 most recent trades (token, action, eth_amount, price/MC at trade, dex,
    /// tx_hash, traded_at). Left as raw JSON — shape is not schema-pinned upstream.
    #[serde(default)]
    pub trades: Vec<serde_json::Value>,
}

// ─── KOL: coordination ───────────────────────────────────────────────────────

/// Query parameters for
/// [`Kol::coordination`](crate::api::kol::Kol::coordination).
#[derive(Debug, Clone, Default, Serialize)]
pub struct CoordinationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<CoordinationPeriod>,
    /// Minimum distinct KOL buyers for a token to qualify (2..=50, default 2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_kols: Option<u32>,
    /// Max tokens (1..=50, default 20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Minimum market cap at the FIRST KOL buy (USD). Tokens with an unknown
    /// entry MC are excluded whenever a band is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mc_usd: Option<f64>,
    /// Maximum market cap at the first KOL buy (USD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mc_usd: Option<f64>,
}

/// One KOL's leg of a coordination cohort.
#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationKol {
    pub evm_address: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub twitter_url: Option<String>,
    pub buy_eth: f64,
    pub sell_eth: f64,
    /// True when this KOL sold more ETH than it bought in the window.
    pub exited: bool,
}

/// A token bought by N+ distinct KOLs in the window, with cohort composition.
#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationToken {
    pub token_address: String,
    #[serde(default)]
    pub token_symbol: Option<String>,
    #[serde(default)]
    pub token_name: Option<String>,
    #[serde(default)]
    pub launchpad: Option<String>,
    #[serde(default)]
    pub is_graduated: Option<bool>,
    #[serde(default)]
    pub deployer_tier: Option<String>,
    #[serde(default)]
    pub token_age_minutes: Option<i64>,
    /// Distinct KOL buyers in the window.
    pub kol_count: i64,
    pub total_buys: i64,
    pub buy_eth: f64,
    pub sell_eth: f64,
    /// `buy_eth − sell_eth` over the window.
    pub net_eth: f64,
    pub signal: CoordinationSignal,
    /// KOLs that sold more than they bought.
    pub exited_count: i64,
    pub holders_count: i64,
    pub first_buy_at: String,
    pub last_buy_at: String,
    /// How fast the cohort piled in, first → last buy.
    pub time_to_consensus_sec: i64,
    #[serde(default)]
    pub market_cap_usd_at_first_buy: Option<f64>,
    #[serde(default)]
    pub current_mc_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_usd: Option<f64>,
    #[serde(default)]
    pub liquidity_usd: Option<f64>,
    /// Per-KOL breakdown, largest buyer first.
    #[serde(default)]
    pub kols: Vec<CoordinationKol>,
}

/// Response of [`Kol::coordination`](crate::api::kol::Kol::coordination).
#[derive(Debug, Clone, Deserialize)]
pub struct CoordinationResponse {
    pub chain: String,
    pub coordination: Vec<CoordinationToken>,
    pub count: u32,
    pub period: String,
    pub min_kols: i64,
}

// ─── KOL: first touches ──────────────────────────────────────────────────────

/// Query parameters for
/// [`Kol::first_touches`](crate::api::kol::Kol::first_touches).
#[derive(Debug, Clone, Default, Serialize)]
pub struct FirstTouchesParams {
    /// Max events (1..=100, default 50). Clamped to 20 below PRO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// ISO 8601 — only first-touches strictly newer. The polling idiom: pass back
    /// the newest timestamp you saw.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// ISO 8601 cursor — only first-touches strictly older. Pass `next_before`
    /// from the previous response to page back.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Minimum first-buy size in ETH (0..=100000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_eth: Option<f64>,
    /// Only tokens younger than N minutes at first touch (1..=43200).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_age_max_min: Option<u32>,
    /// Filter by launchpad: pons, flap, clanker, hood.fun, noxa, virtuals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launchpad: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mc_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mc_usd: Option<f64>,
}

/// The KOL behind a first touch. `evm_address` is ULTRA/BUSINESS only.
#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchKol {
    /// ULTRA/BUSINESS only — omitted on lower tiers.
    #[serde(default)]
    pub evm_address: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub twitter_url: Option<String>,
}

/// The first time ANY tracked KOL bought a given token.
#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchEvent {
    pub token_address: String,
    #[serde(default)]
    pub token_symbol: Option<String>,
    #[serde(default)]
    pub token_name: Option<String>,
    #[serde(default)]
    pub launchpad: Option<String>,
    #[serde(default)]
    pub is_graduated: Option<bool>,
    pub first_buy_at: String,
    #[serde(default)]
    pub eth_amount: Option<f64>,
    #[serde(default)]
    pub token_amount: Option<f64>,
    pub tx_hash: String,
    /// Token age at the first KOL buy, in minutes.
    #[serde(default)]
    pub token_age_minutes: Option<i64>,
    #[serde(default)]
    pub market_cap_usd_at_first_buy: Option<f64>,
    #[serde(default)]
    pub price_usd_at_first_buy: Option<f64>,
    #[serde(default)]
    pub current_mc_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_usd: Option<f64>,
    pub first_kol: FirstTouchKol,
}

/// Response of [`Kol::first_touches`](crate::api::kol::Kol::first_touches).
#[derive(Debug, Clone, Deserialize)]
pub struct FirstTouchesResponse {
    pub chain: String,
    pub events: Vec<FirstTouchEvent>,
    pub count: u32,
    /// Cursor for the next page — pass as `before` to fetch older events.
    #[serde(default)]
    pub next_before: Option<String>,
    #[serde(default)]
    pub data_age_seconds: Option<i64>,
}

// ─── Trades: DEX tape ────────────────────────────────────────────────────────

/// Query parameters for [`Trades::list`](crate::api::trades::Trades::list).
#[derive(Debug, Clone, Default, Serialize)]
pub struct TradesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Filter to one token address (0x, 40 hex).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex: Option<TradeDex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<TradeAction>,
    /// Minimum trade size in ETH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_eth: Option<f64>,
    /// Cursor: trades strictly older than this `block_time`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

/// A single Uniswap swap on Robinhood Chain.
#[derive(Debug, Clone, Deserialize)]
pub struct RhcTrade {
    pub block_number: i64,
    pub block_time: String,
    pub tx_hash: String,
    pub log_index: i64,
    pub dex: String,
    pub pool: String,
    /// Swap-log recipient — the ROUTER for aggregated swaps. Use `trader_eoa`
    /// for wallet analytics.
    #[serde(default)]
    pub trader: Option<String>,
    /// Authoritative trader wallet (tx.from).
    #[serde(default)]
    pub trader_eoa: Option<String>,
    /// Router/aggregator contract (tx.to).
    #[serde(default)]
    pub router: Option<String>,
    #[serde(default)]
    pub token_address: Option<String>,
    #[serde(default)]
    pub action: Option<TradeAction>,
    #[serde(default)]
    pub eth_amount: Option<f64>,
    #[serde(default)]
    pub price_native: Option<f64>,
    #[serde(default)]
    pub price_usd: Option<f64>,
    #[serde(default)]
    pub mc_usd_at_trade: Option<f64>,
    /// Effective gas price, gwei.
    #[serde(default)]
    pub gas_price: Option<f64>,
    /// Transaction position within the block (ordering / sandwich detection).
    #[serde(default)]
    pub tx_index: Option<i64>,
    /// 4-byte calldata selector.
    #[serde(default)]
    pub method_selector: Option<String>,
    /// v3/v4 in-range liquidity at the trade.
    #[serde(default)]
    pub liquidity: Option<f64>,
    #[serde(default)]
    pub launchpad: Option<String>,
    /// True if `trader_eoa` is a tracked KOL wallet.
    #[serde(default)]
    pub is_kol: bool,
    #[serde(default)]
    pub kol_name: Option<String>,
    /// Set if `trader_eoa` is a known deployer.
    #[serde(default)]
    pub deployer_tier: Option<String>,
}

/// Response of [`Trades::list`](crate::api::trades::Trades::list).
#[derive(Debug, Clone, Deserialize)]
pub struct TradesResponse {
    pub chain: String,
    pub trades: Vec<RhcTrade>,
    pub count: u32,
    /// Pagination cursor (last row's `block_time`).
    #[serde(default)]
    pub next_before: Option<String>,
}

// ─── Tokens: discovery ───────────────────────────────────────────────────────

/// Query parameters for [`Tokens::list`](crate::api::tokens::Tokens::list).
#[derive(Debug, Clone, Default, Serialize)]
pub struct TokensListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<TokensSort>,
    /// Minimum current market cap (USD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mc_usd: Option<f64>,
    /// Minimum current liquidity (USD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_liquidity_usd: Option<f64>,
    /// Filter by launchpad: pons, flap, clanker, hood.fun, noxa, virtuals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launchpad: Option<String>,
}

/// A single token in the discovery list.
#[derive(Debug, Clone, Deserialize)]
pub struct RhcTokenSummary {
    pub token_address: String,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub launchpad: Option<String>,
    #[serde(default)]
    pub is_graduated: Option<bool>,
    #[serde(default)]
    pub deployer_address: Option<String>,
    #[serde(default)]
    pub deployer_tier: Option<String>,
    #[serde(default)]
    pub price_usd: Option<f64>,
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    #[serde(default)]
    pub fdv_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_at: Option<String>,
    /// Percent below all-time-high MC.
    #[serde(default)]
    pub drawdown_from_peak_pct: Option<i64>,
    #[serde(default)]
    pub liquidity_usd: Option<f64>,
    #[serde(default)]
    pub primary_dex: Option<String>,
    #[serde(default)]
    pub primary_pool: Option<String>,
    #[serde(default)]
    pub last_trade_time: Option<String>,
}

/// Response of [`Tokens::list`](crate::api::tokens::Tokens::list).
#[derive(Debug, Clone, Deserialize)]
pub struct TokensListResponse {
    pub chain: String,
    pub tokens: Vec<RhcTokenSummary>,
    pub count: u32,
    pub sort: String,
}

// ─── Tokens: single-token bundle snapshot ────────────────────────────────────

/// Deployer reputation block on a token snapshot.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenDeployer {
    pub address: String,
    pub tier: String,
    pub tokens_deployed: i64,
    #[serde(default)]
    pub graduation_rate: Option<f64>,
    #[serde(default)]
    pub runner_rate: Option<f64>,
    pub runners: i64,
    #[serde(default)]
    pub best_peak_mc_usd: Option<f64>,
    #[serde(default)]
    pub launchpads: Vec<String>,
}

/// KOL activity summary on a token snapshot.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenKolActivity {
    pub distinct_kols: i64,
    #[serde(default)]
    pub names: Vec<String>,
    pub buys: i64,
    pub sells: i64,
    pub net_eth: f64,
}

/// Response of [`Tokens::get`](crate::api::tokens::Tokens::get).
#[derive(Debug, Clone, Deserialize)]
pub struct TokenDetailResponse {
    pub chain: String,
    pub token_address: String,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub decimals: Option<i64>,
    #[serde(default)]
    pub launchpad: Option<String>,
    #[serde(default)]
    pub is_graduated: Option<bool>,
    #[serde(default)]
    pub graduated_pool: Option<String>,
    #[serde(default)]
    pub graduated_at: Option<String>,
    #[serde(default)]
    pub deployer_address: Option<String>,
    #[serde(default)]
    pub first_seen_at: Option<String>,
    #[serde(default)]
    pub token_age_minutes: Option<i64>,
    #[serde(default)]
    pub price_usd: Option<f64>,
    #[serde(default)]
    pub price_native: Option<f64>,
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    #[serde(default)]
    pub fdv_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_at: Option<String>,
    #[serde(default)]
    pub drawdown_from_peak_pct: Option<i64>,
    #[serde(default)]
    pub total_supply_raw: Option<String>,
    #[serde(default)]
    pub liquidity_usd: Option<f64>,
    #[serde(default)]
    pub primary_dex: Option<String>,
    #[serde(default)]
    pub primary_pool: Option<String>,
    #[serde(default)]
    pub last_trade_time: Option<String>,
    /// Deployer reputation.
    #[serde(default)]
    pub deployer: Option<TokenDeployer>,
    /// Up to 10 other tokens by the same deployer (symbol or address).
    #[serde(default)]
    pub deployer_other_tokens: Vec<String>,
    #[serde(default)]
    pub kol_activity: Option<TokenKolActivity>,
    /// Up to 20 pools with reserves/liquidity/sqrt_price. Raw JSON — shape is
    /// not schema-pinned upstream.
    #[serde(default)]
    pub pools: Vec<serde_json::Value>,
}

// ─── Tokens: candles ─────────────────────────────────────────────────────────

/// Query parameters for [`Tokens::candles`](crate::api::tokens::Tokens::candles).
#[derive(Debug, Clone, Default, Serialize)]
pub struct CandlesParams {
    /// Number of candles (most recent first, returned chronological). 1..=1000, default 240.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Lower bound on `bucket_start` (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Upper bound on `bucket_start` (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

/// A single 1-minute OHLC candle.
#[derive(Debug, Clone, Deserialize)]
pub struct RhcCandle {
    pub bucket_start: String,
    pub open_price_usd: f64,
    pub high_price_usd: f64,
    pub low_price_usd: f64,
    pub close_price_usd: f64,
    #[serde(default)]
    pub open_mc_usd: Option<f64>,
    #[serde(default)]
    pub high_mc_usd: Option<f64>,
    #[serde(default)]
    pub low_mc_usd: Option<f64>,
    #[serde(default)]
    pub close_mc_usd: Option<f64>,
    #[serde(default)]
    pub close_liquidity_usd: Option<f64>,
    #[serde(default)]
    pub close_supply: Option<f64>,
    pub volume_usd: f64,
    #[serde(default)]
    pub volume_buy_usd: Option<f64>,
    #[serde(default)]
    pub volume_sell_usd: Option<f64>,
    pub trades: i64,
    #[serde(default)]
    pub buy_count: Option<i64>,
    #[serde(default)]
    pub sell_count: Option<i64>,
    #[serde(default)]
    pub dex: Option<String>,
    #[serde(default)]
    pub pool_address: Option<String>,
}

/// Response of [`Tokens::candles`](crate::api::tokens::Tokens::candles).
#[derive(Debug, Clone, Deserialize)]
pub struct CandlesResponse {
    pub chain: String,
    pub token_address: String,
    pub timeframe: String,
    pub candles: Vec<RhcCandle>,
    pub count: u32,
}

// ─── Tokens: KOL consensus ───────────────────────────────────────────────────

/// The KOL cohort's position on a token.
#[derive(Debug, Clone, Deserialize)]
pub struct KolConsensus {
    pub total_kol_buyers: i64,
    pub total_kol_sellers: i64,
    /// Fraction of KOL buyers who also sold (0–1).
    pub kol_exit_rate: f64,
    pub net_flow_eth: f64,
    pub total_buy_eth: f64,
    pub total_sell_eth: f64,
    #[serde(default)]
    pub first_kol_buy_at: Option<String>,
    #[serde(default)]
    pub last_kol_buy_at: Option<String>,
    #[serde(default)]
    pub first_touch_wallet: Option<String>,
    #[serde(default)]
    pub first_touch_at: Option<String>,
    /// Median MC at KOL buy. Null when the RHC pricer withheld MC under its
    /// liquidity gate.
    #[serde(default)]
    pub median_entry_mc_usd: Option<f64>,
    /// Number of buys with a `market_cap_usd_at_trade` — the median's sample size.
    #[serde(default)]
    pub entry_mc_samples: i64,
    pub total_trades: i64,
    /// ULTRA only — distinct KOL buyer wallets.
    #[serde(default)]
    pub buyers: Vec<String>,
    /// ULTRA only — KOL wallets that bought and sold.
    #[serde(default)]
    pub exited: Vec<String>,
}

/// Response of [`Tokens::kol_consensus`](crate::api::tokens::Tokens::kol_consensus).
#[derive(Debug, Clone, Deserialize)]
pub struct KolConsensusResponse {
    pub chain: String,
    pub token_address: String,
    #[serde(default)]
    pub current_mc_usd: Option<f64>,
    #[serde(default)]
    pub current_price_usd: Option<f64>,
    /// Null when no tracked KOL has traded the token.
    #[serde(default)]
    pub consensus: Option<KolConsensus>,
}

// ─── Tokens: buyer quality ───────────────────────────────────────────────────

/// Per-factor breakdown of a buyer-quality read.
#[derive(Debug, Clone, Deserialize)]
pub struct BuyerQualityBreakdown {
    pub early_buyers_analyzed: i64,
    pub alpha_wallet_count: i64,
    pub kol_count: i64,
    /// Early buyers flagged as part of a same-block launch bundle.
    pub bundle_buyer_count: i64,
    /// Early buyers on the rolling dump-cluster list (informational — does not
    /// move the score).
    pub dump_cluster_count: i64,
    /// Early buyers with ≥5 recent early-buyer appearances of any kind.
    pub recycled_early_buyer_count: i64,
    /// Percent (0–100), non-bot buyers with ≥3 tokens of history.
    #[serde(default)]
    pub avg_historical_win_rate: Option<f64>,
    pub bot_dominated: bool,
}

/// The 0–100 buyer-quality read.
#[derive(Debug, Clone, Deserialize)]
pub struct BuyerQuality {
    pub score: i64,
    pub confidence: Confidence,
    pub signal: QualitySignal,
    pub breakdown: BuyerQualityBreakdown,
}

/// Signal-availability block on a buyer-quality response.
#[derive(Debug, Clone, Deserialize)]
pub struct BuyerQualityCoverage {
    #[serde(default)]
    pub bundle_detection: Option<String>,
    #[serde(default)]
    pub dump_cluster_signal: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

/// Response of [`Tokens::buyer_quality`](crate::api::tokens::Tokens::buyer_quality).
#[derive(Debug, Clone, Deserialize)]
pub struct BuyerQualityResponse {
    pub chain: String,
    pub token_address: String,
    #[serde(default)]
    pub current_mc_usd: Option<f64>,
    pub quality: BuyerQuality,
    #[serde(default)]
    pub coverage: Option<BuyerQualityCoverage>,
    /// Present only when buyer data is insufficient.
    #[serde(default)]
    pub note: Option<String>,
}

// ─── Tokens: launch-bundle detection ─────────────────────────────────────────

/// The scalar bundle signal.
#[derive(Debug, Clone, Deserialize)]
pub struct RhcBundle {
    /// Bundle-cohort size (0 when none).
    pub wallet_count: i64,
    /// `same_block` = 3+ early buyers first-bought in one block; `none` = no cluster.
    pub bundle_kind: BundleKind,
    /// Net tokens still held ÷ tokens bought, [0,1]. The primary signal.
    #[serde(default)]
    pub held_ratio: Option<f64>,
    /// Net tokens held ÷ total supply, [0,1]; null when supply is unknown.
    #[serde(default)]
    pub held_pct_of_supply: Option<f64>,
    /// True when the cohort holds ≤0.5% of what it bought.
    pub fully_exited: bool,
    /// Cohort cumulative buy-side token volume (human-scaled).
    pub buy_volume: f64,
    /// Cohort net position (Σbuys − Σsells, human-scaled).
    pub tokens_held: f64,
}

/// A single wallet in the bundle cohort. Empty for BASIC; top-10 for PRO;
/// full cohort for ULTRA. ULTRA-only identity fields default when absent.
#[derive(Debug, Clone, Deserialize)]
pub struct RhcBundleWallet {
    /// Early-buyer rank (1 = first buyer).
    pub rank: i64,
    /// Buyer wallet (lowercase 0x).
    pub wallet: String,
    #[serde(default)]
    pub held_ratio: Option<f64>,
    pub has_sold: bool,
    /// Wallet is a tracked RHC KOL.
    #[serde(default)]
    pub is_kol: bool,
    /// ULTRA only — historical win-rate [0,1].
    #[serde(default)]
    pub win_rate: Option<f64>,
    /// ULTRA only — bot heuristic from `mv_rhc_alpha_wallets`.
    #[serde(default)]
    pub likely_bot: Option<bool>,
    /// ULTRA only — net position, human-scaled.
    #[serde(default)]
    pub tokens_held: Option<f64>,
}

/// Response of [`Tokens::bundle`](crate::api::tokens::Tokens::bundle).
#[derive(Debug, Clone, Deserialize)]
pub struct RhcBundleResponse {
    pub chain: String,
    pub token_address: String,
    pub bundle: RhcBundle,
    /// Empty for BASIC; top-10 for PRO; full cohort for ULTRA.
    #[serde(default)]
    pub wallets: Vec<RhcBundleWallet>,
}

// ─── Tokens: batch ───────────────────────────────────────────────────────────

/// JSON body for the two batch POST endpoints: `{"addresses": [...]}`.
///
/// Built for you by [`Tokens::batch`](crate::api::tokens::Tokens::batch) and
/// [`Tokens::batch_buyer_quality`](crate::api::tokens::Tokens::batch_buyer_quality) —
/// exposed so callers can log or reuse the exact wire body.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchAddressesRequest {
    pub addresses: Vec<String>,
}

/// Deployer reputation block on a batch token entry.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenBatchDeployer {
    /// Deployer wallet (lowercase 0x).
    pub address: String,
    /// How the deployer was attributed (e.g. the launchpad event).
    #[serde(default)]
    pub source: Option<String>,
    /// Present only when the deployer has a reputation row.
    #[serde(default)]
    pub tier: Option<String>,
    #[serde(default)]
    pub tokens_deployed: Option<i64>,
    /// Tokens that reached a $40K+ peak MC.
    #[serde(default)]
    pub graduated: Option<i64>,
    /// The $40K bar. Still returned, but it NO LONGER sets the tier.
    #[serde(default)]
    pub graduation_rate: Option<f64>,
    /// Tokens that peaked ≥ $100K MC.
    #[serde(default)]
    pub runners: Option<i64>,
    /// The $100K bar — this is what `tier` now rides on.
    #[serde(default)]
    pub runner_rate: Option<f64>,
}

/// One token in a batch response. Every REQUESTED address is echoed back, so
/// unknown tokens appear as `found: false` with the rest of the fields absent.
#[derive(Debug, Clone, Deserialize)]
pub struct TokenBatchEntry {
    pub address: String,
    pub found: bool,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub decimals: Option<i64>,
    #[serde(default)]
    pub launchpad: Option<String>,
    #[serde(default)]
    pub is_graduated: Option<bool>,
    #[serde(default)]
    pub graduated_at: Option<String>,
    #[serde(default)]
    pub first_seen_at: Option<String>,
    #[serde(default)]
    pub price_usd: Option<f64>,
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    #[serde(default)]
    pub fdv_usd: Option<f64>,
    #[serde(default)]
    pub liquidity_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_at: Option<String>,
    #[serde(default)]
    pub primary_dex: Option<String>,
    #[serde(default)]
    pub last_trade_time: Option<String>,
    #[serde(default)]
    pub deployer: Option<TokenBatchDeployer>,
}

/// Response of [`Tokens::batch`](crate::api::tokens::Tokens::batch).
#[derive(Debug, Clone, Deserialize)]
pub struct TokenBatchResponse {
    pub chain: String,
    /// One entry per requested address, in the order sent (de-duplicated).
    pub tokens: Vec<TokenBatchEntry>,
    pub requested: i64,
    /// How many of the requested addresses resolved to a known token.
    pub found: i64,
}

/// One token in a batch buyer-quality response.
///
/// Per-token failures degrade to an entry carrying `error` rather than failing
/// the whole batch — one unpriced token never costs you the other 19 results.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchBuyerQualityEntry {
    pub chain: String,
    pub token_address: String,
    #[serde(default)]
    pub current_mc_usd: Option<f64>,
    /// `None` when this token failed to score — see `error`.
    #[serde(default)]
    pub quality: Option<BuyerQuality>,
    /// Present only when buyer data is insufficient.
    #[serde(default)]
    pub note: Option<String>,
    /// Present only when this one token failed to score.
    #[serde(default)]
    pub error: Option<String>,
}

/// Response of
/// [`Tokens::batch_buyer_quality`](crate::api::tokens::Tokens::batch_buyer_quality).
#[derive(Debug, Clone, Deserialize)]
pub struct BatchBuyerQualityResponse {
    pub chain: String,
    pub tokens: Vec<BatchBuyerQualityEntry>,
    pub requested: i64,
    /// Entries that scored (the rest carry `error`).
    pub scored: i64,
    /// The server-side cap — **20**, deliberately lower than the Solana batch
    /// cap of 50 (per-token cohort computation, not one set-based query).
    pub max_addresses: i64,
    #[serde(default)]
    pub coverage: Option<BuyerQualityCoverage>,
}

// ─── Deployer-hunter: leaderboard ────────────────────────────────────────────

/// Query parameters for
/// [`DeployerHunter::leaderboard`](crate::api::deployer_hunter::DeployerHunter::leaderboard).
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeployerLeaderboardParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<DeployerLeaderboardSort>,
    /// Filter to one reputation tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<DeployerTier>,
    /// Minimum tokens deployed (1..=100000, default 3). Note that `elite`/`good`
    /// themselves require ≥5 launches **and** 24h of deployer history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// A single deployer on the reputation leaderboard.
#[derive(Debug, Clone, Deserialize)]
pub struct RhcDeployerRow {
    /// Deployer wallet (lowercase 0x).
    pub deployer_address: String,
    pub tokens_deployed: i64,
    /// Tokens that reached a $40K+ peak MC (the graduation milestone).
    pub graduated: i64,
    /// `graduated ÷ tokens_deployed` — the $40K bar. Still returned, but it no
    /// longer sets `tier` (only `spammer` still keys off it).
    pub graduation_rate: f64,
    /// Tokens that peaked ≥ $100K MC.
    pub runners: i64,
    /// `runners ÷ tokens_deployed` — the $100K bar. This is what `tier` rides
    /// on (migrations 267 + 269).
    pub runner_rate: f64,
    #[serde(default)]
    pub best_peak_mc_usd: Option<f64>,
    #[serde(default)]
    pub launchpads: Vec<String>,
    #[serde(default)]
    pub first_deploy_at: Option<String>,
    #[serde(default)]
    pub last_deploy_at: Option<String>,
    pub tier: String,
}

/// Response of
/// [`DeployerHunter::leaderboard`](crate::api::deployer_hunter::DeployerHunter::leaderboard).
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerLeaderboardResponse {
    pub chain: String,
    pub deployers: Vec<RhcDeployerRow>,
    /// The filtered deployer count; page with limit/offset until `has_more` is false.
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
}

// ─── Deployer-hunter: single profile ─────────────────────────────────────────

/// One deployer's full reputation row.
#[derive(Debug, Clone, Deserialize)]
pub struct RhcDeployer {
    pub deployer_address: String,
    pub tokens_deployed: i64,
    pub curve_tokens: i64,
    pub graduated: i64,
    #[serde(default)]
    pub bonding_rate: Option<f64>,
    pub runners: i64,
    /// `runners ÷ tokens_deployed` — the $100K bar that `tier` rides on.
    pub runner_rate: f64,
    #[serde(default)]
    pub best_peak_mc_usd: Option<f64>,
    #[serde(default)]
    pub launchpads: Vec<String>,
    #[serde(default)]
    pub first_deploy_at: Option<String>,
    #[serde(default)]
    pub last_deploy_at: Option<String>,
    pub tier: String,
}

/// One recent token by a deployer.
#[derive(Debug, Clone, Deserialize)]
pub struct RhcDeployerToken {
    pub address: String,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub launchpad: Option<String>,
    #[serde(default)]
    pub is_graduated: Option<bool>,
    #[serde(default)]
    pub graduated_at: Option<String>,
    #[serde(default)]
    pub graduated_pool: Option<String>,
    #[serde(default)]
    pub first_seen_at: Option<String>,
    /// Live market cap.
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    /// All-time-high MC observed since ingestion.
    #[serde(default)]
    pub peak_mc_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_at: Option<String>,
}

/// Response of
/// [`DeployerHunter::profile`](crate::api::deployer_hunter::DeployerHunter::profile).
///
/// Unknown wallets return 200 with `is_deployer: false` (not a 404).
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerProfileResponse {
    pub chain: String,
    /// False if this wallet has never deployed a tracked token (`deployer` is then null).
    pub is_deployer: bool,
    pub address: String,
    #[serde(default)]
    pub deployer: Option<RhcDeployer>,
    /// Up to 50 most recent tokens by this deployer.
    #[serde(default)]
    pub recent_tokens: Vec<RhcDeployerToken>,
    /// Rows returned (capped at 50) — the true total is `deployer.tokens_deployed`.
    #[serde(default)]
    pub recent_tokens_count: i64,
}

// ─── Deployer-hunter: trajectory ─────────────────────────────────────────────

/// The trailing run at the end of a deployer's (chronological) launch history.
#[derive(Debug, Clone, Deserialize)]
pub struct TrajectoryStreak {
    /// `bond` (a graduation) | `fail` | `none`. Field names keep the Solana
    /// `bond` wording so the two chains stay drop-in compatible.
    #[serde(rename = "type")]
    pub kind: String,
    pub count: i64,
}

/// One rolling 10-launch window.
#[derive(Debug, Clone, Deserialize)]
pub struct TrajectoryWindow {
    /// 1-based index of the launch that closes this window.
    pub window_end: i64,
    /// Share of the window's launches that graduated (0–1).
    pub bond_rate: f64,
}

/// The best or worst rolling 10-launch stretch.
#[derive(Debug, Clone, Deserialize)]
pub struct TrajectoryStretch {
    pub start_index: i64,
    pub end_index: i64,
    pub bond_rate: f64,
}

/// A deployer's launch quality over time.
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerTrajectory {
    pub current_streak: TrajectoryStreak,
    pub longest_bond_streak: i64,
    pub longest_fail_streak: i64,
    /// Rolling 10-launch success rates, oldest window first.
    #[serde(default)]
    pub rolling_bond_rates: Vec<TrajectoryWindow>,
    /// Latest rolling rate vs the lifetime rate, ±0.05.
    pub trend: TrajectoryTrend,
    #[serde(default)]
    pub avg_days_between_deploys: Option<f64>,
    /// Average launches burned between a miss and the next hit.
    #[serde(default)]
    pub avg_recovery_tokens: Option<f64>,
    #[serde(default)]
    pub best_stretch: Option<TrajectoryStretch>,
    #[serde(default)]
    pub worst_stretch: Option<TrajectoryStretch>,
    pub total_tokens_analyzed: i64,
}

/// Response of
/// [`DeployerHunter::trajectory`](crate::api::deployer_hunter::DeployerHunter::trajectory).
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerTrajectoryResponse {
    pub chain: String,
    /// False if this wallet has never deployed a tracked token.
    pub is_deployer: bool,
    pub address: String,
    #[serde(default)]
    pub deployer: Option<RhcDeployerRow>,
    /// What was actually counted per token: the $40K peak-MC graduation
    /// milestone — NOT the $100K runner bar that TIERS ride on.
    #[serde(default)]
    pub success_metric: Option<String>,
    /// `None` for an unknown wallet.
    #[serde(default)]
    pub trajectory: Option<DeployerTrajectory>,
    /// True when the analysis hit the 500-launch cap, so the curve is not the
    /// whole history.
    #[serde(default)]
    pub truncated: bool,
}

// ─── Deployer-hunter: launch history (paginated) ─────────────────────────────

/// Query parameters for
/// [`DeployerHunter::tokens`](crate::api::deployer_hunter::DeployerHunter::tokens).
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeployerTokensParams {
    /// Page size (1..=100, default 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Page offset (0..=10000, default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<DeployerTokensSort>,
}

/// One token in a deployer's paginated launch history.
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerTokenRow {
    pub address: String,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub launchpad: Option<String>,
    /// How the deployer was attributed for this token.
    #[serde(default)]
    pub deployer_source: Option<String>,
    #[serde(default)]
    pub is_graduated: Option<bool>,
    #[serde(default)]
    pub graduated_at: Option<String>,
    #[serde(default)]
    pub first_seen_at: Option<String>,
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_at: Option<String>,
    #[serde(default)]
    pub liquidity_usd: Option<f64>,
}

/// Response of
/// [`DeployerHunter::tokens`](crate::api::deployer_hunter::DeployerHunter::tokens).
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerTokensResponse {
    pub chain: String,
    pub is_deployer: bool,
    pub address: String,
    #[serde(default)]
    pub deployer: Option<RhcDeployerRow>,
    #[serde(default)]
    pub tokens: Vec<DeployerTokenRow>,
    /// The deployer's lifetime launch count — page until `has_more` is false.
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
    #[serde(default)]
    pub sort: Option<String>,
    /// `"page"` when `sort = peak_mc_usd` — the ordering was applied to this page
    /// only, not the whole history.
    #[serde(default)]
    pub sort_scope: Option<String>,
}

// ─── Deployer-hunter: best tokens ────────────────────────────────────────────

/// Query parameters for
/// [`DeployerHunter::best_tokens`](crate::api::deployer_hunter::DeployerHunter::best_tokens).
#[derive(Debug, Clone, Default, Serialize)]
pub struct BestTokensParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<BestTokensPeriod>,
    /// Max tokens (1..=50, default 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Reputation block on a best-tokens row.
#[derive(Debug, Clone, Deserialize)]
pub struct BestTokenDeployer {
    pub address: String,
    /// `elite` or `good` — earned on the $100K `runner_rate`.
    pub tier: String,
    /// The $40K bar. Returned, but it no longer sets the tier.
    #[serde(default)]
    pub graduation_rate: Option<f64>,
    #[serde(default)]
    pub runner_rate: Option<f64>,
    pub tokens_deployed: i64,
}

/// A token launched in the window by a currently-reputable deployer.
#[derive(Debug, Clone, Deserialize)]
pub struct BestToken {
    pub address: String,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub launchpad: Option<String>,
    #[serde(default)]
    pub first_seen_at: Option<String>,
    #[serde(default)]
    pub is_graduated: Option<bool>,
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_at: Option<String>,
    #[serde(default)]
    pub liquidity_usd: Option<f64>,
    #[serde(default)]
    pub deployer: Option<BestTokenDeployer>,
}

/// Response of
/// [`DeployerHunter::best_tokens`](crate::api::deployer_hunter::DeployerHunter::best_tokens).
#[derive(Debug, Clone, Deserialize)]
pub struct BestTokensResponse {
    pub chain: String,
    /// Ranked by peak MC, descending.
    pub tokens: Vec<BestToken>,
    pub period: String,
    pub limit: i64,
    /// Size of the elite/good deployer population the window was drawn from.
    pub reputable_deployers: i64,
    /// Launches considered before ranking. Absent when the population was empty.
    #[serde(default)]
    pub candidates_scanned: Option<i64>,
    /// True when the top-N was drawn from the 1000 most RECENT launches in the
    /// period rather than the whole period.
    #[serde(default)]
    pub truncated: bool,
}

// ─── Deployer-hunter: chain-wide stats ───────────────────────────────────────

/// Population of one reputation tier.
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerTierCount {
    pub deployers: i64,
    pub tokens: i64,
}

/// The ACTIVE tier thresholds, so you can see what `elite` currently means
/// instead of guessing from the label.
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerTierRules {
    /// Earned on the $100K `runner_rate` (migrations 267 + 269).
    #[serde(default)]
    pub elite: Option<String>,
    #[serde(default)]
    pub good: Option<String>,
    /// The one tier still keyed off `graduation_rate` — detecting trash is a
    /// different question from detecting quality.
    #[serde(default)]
    pub spammer: Option<String>,
    #[serde(default)]
    pub neutral: Option<String>,
}

/// Response of
/// [`DeployerHunter::stats`](crate::api::deployer_hunter::DeployerHunter::stats).
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerStatsResponse {
    pub chain: String,
    pub total_deployers: i64,
    pub total_tokens: i64,
    /// `elite` + `good` deployers.
    pub reputable_deployers: i64,
    /// Tier name → population.
    #[serde(default)]
    pub by_tier: HashMap<String, DeployerTierCount>,
    /// Share of all indexed tokens deployed by `spammer`-tier wallets.
    #[serde(default)]
    pub spam_token_share: Option<f64>,
    pub alerts_24h: i64,
    pub alerts_7d: i64,
    pub tier_rules: DeployerTierRules,
    /// `"peak market cap >= $40,000"`.
    pub graduation_definition: String,
    /// `"peak market cap >= $100,000"`.
    pub runner_definition: String,
}

// ─── Deployer-hunter: alerts ─────────────────────────────────────────────────

/// Query parameters for
/// [`DeployerHunter::alerts`](crate::api::deployer_hunter::DeployerHunter::alerts).
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeployerAlertsParams {
    /// Matched against the READ-TIME `tier`, not the snapshot, so the filter and
    /// the payload always agree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployer_tier: Option<DeployerTier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<DeployerAlertPriority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_type: Option<DeployerAlertType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launchpad: Option<String>,
    /// Minimum market cap at alert time (USD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mc: Option<f64>,
    /// `true` disables the default $100 liquidity floor and returns the raw tape.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_untradeable: Option<bool>,
    /// ISO 8601 — only alerts with `event_at` strictly newer. The
    /// incremental-polling idiom: pass back `next_event_at`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// ISO 8601 cursor — only alerts strictly older. Takes precedence over
    /// `offset`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Max alerts (1..=500, default 50). BASIC/PRO are capped at 50; only ULTRA
    /// gets the full requested limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Page offset (0..=10000, default 0). Ignored when `before` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// A single deployer alert.
#[derive(Debug, Clone, Deserialize)]
pub struct RhcDeployerAlert {
    pub id: String,
    pub deployer_address: String,
    #[serde(default)]
    pub token_address: Option<String>,
    #[serde(default)]
    pub token_symbol: Option<String>,
    #[serde(default)]
    pub token_name: Option<String>,
    pub alert_type: DeployerAlertType,
    #[serde(default)]
    pub title: Option<String>,
    /// Restated at read time in terms of the $100K runner rate for elite/good
    /// deployers, replacing snapshot copy that quoted the $40K graduation rate.
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub launchpad: Option<String>,
    /// The deployer's CURRENT tier, resolved at read time — not the snapshot.
    #[serde(default)]
    pub tier: Option<String>,
    /// The snapshot written when the alert fired.
    #[serde(default)]
    pub tier_at_alert: Option<String>,
    /// True when `tier` and `tier_at_alert` disagree.
    #[serde(default)]
    pub tier_is_stale: bool,
    #[serde(default)]
    pub mc_at_alert: Option<f64>,
    #[serde(default)]
    pub current_mc_usd: Option<f64>,
    /// Live liquidity — what the tradability filter gates on.
    #[serde(default)]
    pub liquidity_usd: Option<f64>,
    pub priority: DeployerAlertPriority,
    pub is_active: bool,
    pub created_at: String,
    /// When the on-chain event actually happened. This is the ordering key.
    #[serde(default)]
    pub event_at: Option<String>,
}

/// Response of
/// [`DeployerHunter::alerts`](crate::api::deployer_hunter::DeployerHunter::alerts).
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerAlertsResponse {
    pub chain: String,
    pub alerts: Vec<RhcDeployerAlert>,
    pub limit: i64,
    pub offset: i64,
    /// The active setting, echoed back — `"liquidity_usd >= $100"` or
    /// `"off (include_untradeable=true)"`.
    pub tradability_filter: String,
    /// Newest `event_at` on this page — pass as `since` to poll forward.
    #[serde(default)]
    pub next_event_at: Option<String>,
    /// Oldest `event_at` on this page — pass as `before` to page backwards.
    #[serde(default)]
    pub next_before: Option<String>,
    #[serde(default)]
    pub data_age_seconds: Option<i64>,
}

// ─── Deployer-hunter: deploy history (PRO+) ──────────────────────────────────

/// Query parameters for
/// [`DeployerHunter::history`](crate::api::deployer_hunter::DeployerHunter::history).
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeployerHistoryParams {
    /// Page size (1..=1000, default 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Page offset (0..=100000, default 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// One token in a deployer's deploy history.
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerHistoryToken {
    pub address: String,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub launchpad: Option<String>,
    #[serde(default)]
    pub is_graduated: Option<bool>,
    #[serde(default)]
    pub graduated_at: Option<String>,
    #[serde(default)]
    pub graduated_pool: Option<String>,
    #[serde(default)]
    pub first_seen_at: Option<String>,
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_at: Option<String>,
}

/// Response of
/// [`DeployerHunter::history`](crate::api::deployer_hunter::DeployerHunter::history).
#[derive(Debug, Clone, Deserialize)]
pub struct DeployerHistoryResponse {
    pub chain: String,
    pub is_deployer: bool,
    pub address: String,
    #[serde(default)]
    pub deployer: Option<RhcDeployerRow>,
    /// Newest first, with a stable tiebreaker so pages never overlap or skip.
    #[serde(default)]
    pub tokens: Vec<DeployerHistoryToken>,
    /// Exact count.
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
}

// ─── Deployer-hunter: recent graduations ─────────────────────────────────────

/// Query parameters for
/// [`DeployerHunter::recent_bonds`](crate::api::deployer_hunter::DeployerHunter::recent_bonds).
#[derive(Debug, Clone, Default, Serialize)]
pub struct RecentBondsParams {
    /// Filter by the deployer's tier. Tiers ride the $100K runner rate, not the
    /// $40K graduation bar this feed is keyed on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployer_tier: Option<DeployerTier>,
    /// Raise the peak-MC floor (USD). Never lowers it below $40K.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_peak: Option<f64>,
    /// Max tokens (1..=200, default 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// A token that crossed the $40K peak-MC graduation milestone.
#[derive(Debug, Clone, Deserialize)]
pub struct RecentBondToken {
    pub address: String,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub launchpad: Option<String>,
    #[serde(default)]
    pub is_graduated: Option<bool>,
    #[serde(default)]
    pub deployer_address: Option<String>,
    #[serde(default)]
    pub deployer_tier: Option<String>,
    #[serde(default)]
    pub first_seen_at: Option<String>,
    #[serde(default)]
    pub market_cap_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_usd: Option<f64>,
    #[serde(default)]
    pub peak_mc_at: Option<String>,
}

/// Response of
/// [`DeployerHunter::recent_bonds`](crate::api::deployer_hunter::DeployerHunter::recent_bonds).
#[derive(Debug, Clone, Deserialize)]
pub struct RecentBondsResponse {
    pub chain: String,
    /// The graduation milestone in USD — 40000.
    pub graduation_mc: i64,
    /// Newest peak first.
    pub tokens: Vec<RecentBondToken>,
    pub limit: i64,
    /// Newest peak timestamp on this page (informational).
    #[serde(default)]
    pub next_peak_mc_at: Option<String>,
}

// ─── Alpha wallets ───────────────────────────────────────────────────────────

/// Query parameters for
/// [`AlphaWallets::list`](crate::api::alpha_wallets::AlphaWallets::list).
#[derive(Debug, Clone, Default, Serialize)]
pub struct AlphaWalletsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<AlphaClassification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<AlphaIdentity>,
    /// Minimum share of trades in launchpad memecoins (0–1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_memecoin_share: Option<f64>,
    /// Maximum average market cap traded — filter to low-cap degens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_avg_mc_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_net_eth: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_win_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_win_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_trades: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_tokens: Option<u32>,
    /// Minimum ETH deployed (whale/size filter).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_buy_eth: Option<f64>,
    /// Only wallets that traded within the last N hours (1..=720).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_hours: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<AlphaSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// A single smart-money wallet on Robinhood Chain.
#[derive(Debug, Clone, Deserialize)]
pub struct RhcAlphaWallet {
    /// Trader EOA (lowercase 0x).
    pub wallet: String,
    /// `bot` | `smart_money` | `trader`.
    pub classification: String,
    pub is_known_kol: bool,
    pub trades: i64,
    pub tokens: i64,
    pub buy_eth: f64,
    pub sell_eth: f64,
    /// Realized net flow (sell − buy).
    pub net_eth: f64,
    #[serde(default)]
    pub win_rate: Option<f64>,
    /// Share of trades in launchpad memecoins (vs tokenized stocks/stables).
    #[serde(default)]
    pub memecoin_share: Option<f64>,
    #[serde(default)]
    pub avg_trade_mc_usd: Option<f64>,
    #[serde(default)]
    pub last_trade_at: Option<String>,
}

/// Response of
/// [`AlphaWallets::list`](crate::api::alpha_wallets::AlphaWallets::list).
#[derive(Debug, Clone, Deserialize)]
pub struct AlphaWalletsResponse {
    pub chain: String,
    pub wallets: Vec<RhcAlphaWallet>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
}

// ─── Streaming ───────────────────────────────────────────────────────────────

/// A 24-hour WebSocket streaming token, as returned by
/// [`Stream::get_token`](crate::api::stream::Stream::get_token).
///
/// Subscribe to the Robinhood Chain channels [`RHC_KOL_TRADES`](crate::api::stream::RHC_KOL_TRADES)
/// and [`RHC_TRADES`](crate::api::stream::RHC_TRADES) after connecting to `ws_url`.
#[derive(Debug, Clone, Deserialize)]
pub struct StreamToken {
    pub token: String,
    pub expires_at: String,
    #[serde(default)]
    pub next_refresh_at: Option<String>,
    pub ws_url: String,
    /// Only present for ULTRA-tier subscribers.
    #[serde(default)]
    pub dex_ws_url: Option<String>,
    #[serde(default)]
    pub usage: Option<String>,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

/// A single live WebSocket session for your account.
#[derive(Debug, Clone, Deserialize)]
pub struct StreamSession {
    /// Numeric session id (serialized as a string). Pass to
    /// [`Stream::kill_session`](crate::api::stream::Stream::kill_session).
    pub id: String,
    /// Originating service: `"ws-streaming"` or `"dex-stream"`.
    pub service: String,
    pub tier: String,
    pub channels: Vec<String>,
    pub connected_at: String,
    #[serde(default)]
    pub remote_ip: Option<String>,
    /// Messages pushed to this socket so far.
    pub messages_sent: u64,
}

/// Response of [`Stream::sessions`](crate::api::stream::Stream::sessions).
#[derive(Debug, Clone, Deserialize)]
pub struct StreamSessionsResponse {
    pub sessions: Vec<StreamSession>,
    pub count: u32,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}

/// Response of [`Stream::kill_session`](crate::api::stream::Stream::kill_session).
#[derive(Debug, Clone, Deserialize)]
pub struct StreamSessionEvicted {
    pub evicted: bool,
    pub id: String,
    #[serde(default, rename = "_rid")]
    pub _rid: Option<String>,
}
