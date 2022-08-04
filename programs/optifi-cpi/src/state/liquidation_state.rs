use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Debug)]
#[repr(u8)]
pub enum LiquidationStatus {
    Healthy,
    CancelOrder,
    PlaceOrder,
    SettleOrder,
}

impl Default for LiquidationStatus {
    fn default() -> Self {
        LiquidationStatus::Healthy
    }
}

#[account]
#[derive(Default)]
pub struct LiquidationState {
    pub user_account: Pubkey,      // 32 bytes
    pub status: LiquidationStatus, // 1 bytes
    pub timestamp: u64,            // 8 bytes
    pub markets: Vec<Pubkey>,      // 36 * 32 bytes
    pub values: Vec<i64>,          // 36 * 8 bytes
}

impl LiquidationState {
    /// calc the space to be allocated
    pub fn get_space_allocation() -> usize {
        1600 // 32+1+8+36*32+32*8
    }
}
