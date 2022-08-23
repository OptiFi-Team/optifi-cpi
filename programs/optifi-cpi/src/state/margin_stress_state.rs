use crate::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct MarginStressAccount {
    /// optifi exchange which the MarginStress belongs to
    pub optifi_exchange: Pubkey,
    /// bump seed used to derive this MarginStress address
    pub bump: u8,
    /// underlying asset
    pub asset: Asset, // 1 bytes

    pub spot_price: u64, // f_to_u_repr
    pub iv: u64,         // f_to_u_repr

    pub timestamp: u64,

    /// MarginStress's state indicator
    pub state: MarginStressState,
    /// each instrument's state flag under the current MarginStress state
    pub flags: Vec<bool>,

    /// a list of pubkeys of the instruments
    pub instruments: Vec<Pubkey>,
    pub strikes: Vec<u64>,
    pub is_call: Vec<u8>,
    pub expiry_date: Vec<u64>,

    pub option_price: Vec<u64>,
    pub intrinsic_value: Vec<u64>,
    pub option_price_delta_in_stress_price: Vec<Vec<i64>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AnchorDeserialize, AnchorSerialize)]
pub enum MarginStressState {
    Sync,
    Calculate,
    Available,
}

impl Default for MarginStressState {
    fn default() -> MarginStressState {
        MarginStressState::Sync
    }
}

impl MarginStressAccount {
    /// calc the space to be allocated
    pub fn get_space_allocation() -> usize {
        3000
    }

    // check if the data in ms account is timeout
    pub fn is_timeout(&self) -> bool {
        let ms_time = self.timestamp;
        let now = Clock::get().unwrap().unix_timestamp as u64;
        now - ms_time > MS_TIMEOUT_TOLERANCE
    }
}
