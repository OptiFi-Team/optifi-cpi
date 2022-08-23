use crate::prelude::*;

#[account]
#[derive(Debug)]
pub struct FeeAccount {
    pub user_account: Pubkey,         // 32
    pub notional_trading_volume: u64, // 8
    pub acc_fee: u64,                 // 8
    pub open_order_fee: Vec<Fee>,     // 16 * 300
}

#[derive(Debug, Copy, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct Fee {
    pub spot_price: u64,      // 8
    pub client_order_id: u64, // 8
}

impl FeeAccount {
    /// calc the space to be allocated
    pub fn get_space_allocation() -> usize {
        32 + 8 + 8 + 16 * 300
    }
}
