/// Important constants used throughout the system

#[cfg(not(feature = "devnet"))]
mod mainnet;
#[cfg(not(feature = "devnet"))]
pub use mainnet::*;

#[cfg(feature = "devnet")]
mod devnet;
#[cfg(feature = "devnet")]
pub use devnet::*;

// Referral account
// pub const REFERRAL_USDC: &str = "";

// The fee (in USD) that a cranker will receive.
pub const CRANKER_FEE: f64 = 0.002;
pub const MM_BALANCE_THRESHOLD: f64 = 0.1;

// Current version of the market schema
pub const MARKET_VERSION: i32 = 1;

// Orderbook spread limit for penalties, 1%
pub const SPREAD_LIMIT: f64 = 0.01;

/// How many strikes to generate on either side of a spot,
/// and the increment in USD they'll be generated at.
/// This value MUST be odd for the strike ladder to be generated
/// correctly
/// ```rust
/// use optifi::constants::STRIKES;
/// assert_eq!(STRIKES % 2, 1)
/// ```
pub const STRIKES: i32 = 5;
pub const BACKUP_STRIKES: i32 = 4;
// This will be the number of strikes on either side of the
// atm strike.
pub const LADDER_SIZE: i32 = (STRIKES - 1) / 2;
pub const BTC_STRIKES_INCR_USD: i32 = 500;
pub const ETH_STRIKES_INCR_USD: i32 = 50;

pub const USDC_DECIMALS: u32 = 6u32;

// Constant for the AMM
pub const DELTA_LIMIT: f64 = 0.05; // delta limit for hedging
pub const TRADE_CAPACITY: f64 = 0.25;
pub const NSTEP: i64 = 25; //nStep to generate orderbook
pub const NQUOTES: i64 = 5; //to place on orderbook
pub const MAX_ORDERBOOK_SIZE: f64 = 10.0;
pub const PRICE_MOVE: f64 = 0.005; // price change tolerance for updating amm orders
pub const ORDER_LEVELS: usize = 20;
pub const UPDATE_SECS: u64 = 2 * 60;
pub const COST_FUT: f64 = 0.0001;
pub const MIN_PRICE_RATIO: f64 = 0.0005;

// Some useful datetime constants
pub const SECONDS_IN_MINUTE: u64 = 60;
pub const MINUTES_IN_HOUR: u64 = 60;
pub const HOURS_IN_DAY: u64 = 24;
pub const DAYS_IN_WEEK: u64 = 7;
pub const DAYS_IN_STANDARD_YEAR: u64 = 365;
pub const SECS_IN_HOUR: u64 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;
pub const SECS_IN_DAY: u64 = SECS_IN_HOUR * HOURS_IN_DAY;
pub const SECS_IN_STANDARD_YEAR: u64 = SECS_IN_DAY * DAYS_IN_STANDARD_YEAR;

// Constant for the margin calculation
pub const STRESS: f64 = 0.3;
pub const STEP: u8 = 5;

// Constant for liquidation
pub const LIQUIDATION: f64 = 0.9;
pub const LIQUIDATION_TO_AMM: f64 = 0.5;
pub const HEALTHY_RATIO: f64 = 1.3;

// Constant for market maker (mm)
pub const FEE_SHARE: f64 = 0.5;
pub const MM_REQ: f64 = 0.1; // MM balance threshold

// Margin stress result timeout tolerance in seconds
pub const MS_TIMEOUT_TOLERANCE: u64 = 20;

// Weekly Fee Log
pub const WEEKLY_LOG_LEN: usize = 4;
