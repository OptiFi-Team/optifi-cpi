use crate::prelude::*;

#[account]
#[derive(Default)]
pub struct AmmAccount {
    /// optifi exchange which the AMM belongs to
    pub optifi_exchange: Pubkey,
    /// index of the amm
    pub amm_idx: u8,
    /// quote token mint address
    pub quote_token_mint: Pubkey,
    /// quote token account address
    pub quote_token_vault: Pubkey,
    /// LP tokens for liquidity providers
    pub lp_token_mint: Pubkey,
    pub amm_capacity: u64,
    /// bump seed used to derive this amm address
    pub bump: u8,
    /// underlying asset
    pub asset: u8, // 1 bytes
    /// a list of pubkeys of the instruments AMM will trade
    pub trading_instruments: Vec<Pubkey>,
    /// a list of Position struct by instruments
    // pub positions: [Position; (STRIKES + BACKUP_STRIKES) as usize * 2],
    pub positions: Vec<Position>,
    /// a list of proposals of orders to excute for each instrument
    // pub proposals: [Proposal; (STRIKES + BACKUP_STRIKES) as usize * 2],
    pub proposals: Vec<Proposal>,
    /// amm's state indicator
    pub state: AmmState,
    /// each instrument's state flag under the current AMM state
    // pub flags: [bool; (STRIKES + BACKUP_STRIKES) as usize * 2],
    /// the first flag is for future, while remaining flags for options
    pub flags: Vec<bool>,
    /// the implied volatility
    pub iv: u64,
    /// the underlying asset price denominated in USDC
    pub price: u64,
    /// the net delta denominated in underlying asset
    pub net_delta: i64,
    /// the timestamp when lastest update
    pub timestamp: u64,
    /// the amm total liquidity denominated in USDC (with 6 decimals)
    pub total_liquidity_usdc: u64,
    /// the duration type (Weekly/Monthly)
    pub duration: Duration,
    /// the expiry date
    pub expiry_date: u64,
    /// the contract size *10000 (f_to_u_repr)
    pub contract_size: u64,
    /// client order id counter for amm placing orders
    pub client_order_id_counter: u64,
    /// amm's mango account for hedging on mango market
    pub amm_mango_account: Pubkey,
    /// is mango hedging needed
    pub is_hedge_needed: bool,
    /// is mango hedging in progress
    pub is_hedge_in_progress: bool,
    /// temp PnL record for fund settlment purpose
    pub temp_pnl: TempPnL, // 16 bytes
    /// pubkey of amm's withdraw queue
    pub withdraw_queue: Pubkey,
}

/// 1. When AMM state is Sync. a syncing cranker finds instrument with false flag to sync positions.
///    If all flag are true, AMM state will change to Calculate, and flags will be reset to all false.
/// 2. When AMM state is Calculate, a calculating crankers instrument finds instrument with false to calc
///    and save the proposal, and set this instrument's flag to true.
///    If all flags are true, AMM state will change to Execute, and flags will be reset to all false.
/// 3. When AMM state is Execute, the first executing cranker find those instuments with false flags to
///    execute the orders in proposal.
///    In a propsal, if the flag is_started is false, so the cranker, as the first cranker will need
///    to cancel the previous orders of this instrumnet, and then submit some orders in orders_to_execute of the proposal.
///    If the length of orders_to_execute is 0, which means the proposal is finished. so the cranker will
///    set instrument's flag in AMM as true.
///    If all flags in AMM are true, the executing cranker will change AMM state into Sync and
///    reset all flags in AMM to false, which means next round of AMM update can be started.
#[derive(Clone, Copy, PartialEq, Eq, AnchorDeserialize, AnchorSerialize)]
pub enum AmmState {
    Sync,
    CalculateDelta,
    CalculateProposal,
    Execute,
}

impl Default for AmmState {
    fn default() -> AmmState {
        AmmState::Sync
    }
}

impl AmmAccount {
    /// calc the space to be allocated
    pub fn get_space_allocation() -> usize {
        // 32 * 6
        //     + 1 * 3
        //     + 8 * 10
        //     + 1
        //     + 1
        //     + 16
        //     + (32 + (32 * 3 + 8 * 2) + (32 + 1 + 8 * 8 * 5) + 1) * strike * 2
        // = 6269 for 6 strikes
        7000
    }
}

#[derive(Default, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Proposal {
    /// instrument pubkey
    pub instrument: Pubkey,
    /// if the orders execution is started
    pub is_started: bool,
    /// all ask_orders_size
    pub ask_orders_size: Vec<u64>,
    /// all orders to execute
    pub bid_orders_size: Vec<u64>,
    /// all orders to execute
    pub ask_orders_price: Vec<u64>,
    /// all orders to execute
    pub bid_orders_price: Vec<u64>,
    /// ask order ids of last round
    pub ask_client_order_ids: Vec<u64>,
    /// bid order ids of last round
    pub bid_client_order_ids: Vec<u64>,
    /// ask order prices of last round
    pub prev_ask_orders_price: Vec<u64>,
    /// bid order prices of last round
    pub prev_bid_orders_price: Vec<u64>,
}

#[derive(Default, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub struct Position {
    /// the instrument of the positions
    pub instruments: Pubkey,
    /// base token account address
    pub long_token_vault: Pubkey,
    /// base token account address
    pub short_token_vault: Pubkey,
    /// latest position updated by update_amm_positions
    pub latest_position: i64,
    /// the usdc remains in the vault
    pub usdc_balance: u64,
}
