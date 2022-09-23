use super::SECS_IN_DAY;

// mainnet

// The fee for each transaction on the OptiFi system
pub const TAKER_FEE: f64 = 0.0010;
pub const SERUM_TAKER_FEE: f64 = 0.0004;
pub const OPTIFI_TAKER_FEE: f64 = 0.0006;

// PostOnly order
pub const MAKER_FEE: f64 = 0.0006;
pub const SERUM_MAKER_FEE: f64 = 0.0;
pub const OPTIFI_MAKER_FEE: f64 = 0.0006;
pub const REGISTER_MAKER_FEE: f64 = 0.0;

// Constant mango program addressses on mainnet
pub const MANGO_PROGRAM_ID: &str = "mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68";

// AMM
pub const AMM_WITHDRAW_WAITING_TIME: u64 = SECS_IN_DAY * 2; // 2 days in seconds

pub const OPTIFI_EXCHANGE: &str = "575NEMoeiqA3moqAgNy9iqxDwUfjGCm92NZoCW8xS9C9";
pub const USDC_TOKEN_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

pub const SERUM_DEX_PROGRAM_ID: &str = "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin";
