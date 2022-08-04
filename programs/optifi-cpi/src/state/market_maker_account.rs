use crate::prelude::*;

#[account]
#[derive(Default)]
pub struct MarketMakerAccount {
    /// The user that this market maker is associated with
    pub user_account: Pubkey,
    pub active: bool,
    /// This is used for the market maker 24 hour withdrawl window - if this is 0, then
    /// there's no withdrawal currently registered. If it's not 0, it's the timestamp at which
    /// a withdrawal was started
    pub withdraw_ts: u64,
    pub withdrawal_amount: u64,

    pub open_orders_data: Vec<OpenOrdersData>,
}
impl MarketMakerAccount {
    /// calc the space to be allocated
    pub fn get_space_allocation() -> usize {
        10240
    }
}

#[derive(Clone, Default, AnchorSerialize, AnchorDeserialize)]
pub struct OpenOrdersData {
    pub market: Pubkey,

    pub timestamp: u64,

    pub bids: Vec<Order>,
    pub asks: Vec<Order>,

    pub ask_1: u64,      // f_to_u
    pub bid_1: u64,      // f_to_u
    pub spot_price: u64, // f_to_u
    pub delta: i64,      // f_to_i

    pub margin: u64,   // f_to_u
    pub position: i64, // f_to_i

    //
    pub state: MmState,

    // Stored information for rolling MM calculations
    pub volume: u64,
    pub total_indicator: u64,         // f_to_u
    pub time_weighted_indicator: u64, // f_to_u
    pub daily_volume: u64,            // f_to_u
    pub penalty: u64,                 // f_to_u

    pub ask_reward: u64, // f_to_u
    pub bid_reward: u64, // f_to_u
}

#[derive(Debug, Clone, Default, AnchorSerialize, AnchorDeserialize)]
pub struct Order {
    // pub timestamp: u64,
    pub price: u64,
    pub size: u64,
    pub client_order_id: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, AnchorDeserialize, AnchorSerialize)]
pub enum MmState {
    Calculation,
    Available,
}

impl Default for MmState {
    fn default() -> MmState {
        MmState::Available
    }
}
