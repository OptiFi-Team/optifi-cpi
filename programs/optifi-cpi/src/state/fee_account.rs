use crate::prelude::*;

#[account]
pub struct FeeAccount {
    pub user_account: Pubkey,         // 32
    pub fee_tier: FeeTier,            // 1
    pub notional_trading_volume: u64, // 8
    pub acc_fee: u64,                 // 8
    pub referrer: Option<Pubkey>,     // 33
    pub acc_rebate_fee: u64,          // 8
    pub open_order_fee: Vec<FeeLog>,  // 32 * 300
    pub referree_num: u64,
    pub weekly_log: [WeeklyLog; WEEKLY_LOG_LEN],
}

#[derive(Debug, Copy, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct FeeLog {
    pub max_coin_qty: u64,    // 8
    pub fee: u64,             // 8
    pub client_order_id: u64, // 8
    pub spot_price: u64,      // 8
}

#[derive(Default, Debug, Copy, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct WeeklyLog {
    pub expiry_date: u64,
    pub weekly_acc_fee: u64,
    pub weekly_notional_trading_volume: u64,
}

impl FeeAccount {
    /// calc the space to be allocated
    pub fn get_space_allocation() -> usize {
        8 + 32 + 1 + 8 + 8 + 33 + 8 + 32 * 300
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum FeeTier {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
}
