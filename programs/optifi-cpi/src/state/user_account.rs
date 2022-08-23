use crate::prelude::*;

#[account]
#[derive(Debug)]
pub struct UserAccount {
    /// optifi exchange which the user account belongs to
    pub optifi_exchange: Pubkey, // 32 bytes

    /// The owner of this account.
    pub owner: Pubkey, // 32 bytes

    /// user account id
    pub id: u64,
    /// The margin account which user deposits usdc token into
    /// it's a spl token account
    pub user_margin_account_usdc: Pubkey, // 32 bytes

    /// temp PnL record for fund settlment purpose
    pub temp_pnl: TempPnL, // 16 bytes

    // /// The total amount of tokens the user deposited into this account.
    // pub reserve: u64,
    /// The account's state
    pub state: AccountState, // 1 bytes

    /// a list of instrument Pubkey and position
    pub positions: Vec<UserPosition>, // 36 * (32+8*2) bytes

    pub is_in_liquidation: bool, // 1 bytes

    pub is_market_maker: bool, // 1 bytes

    /// the bump seed to get the address of this user account
    pub bump: u8, // 1 bytes

    /// margin requirement
    pub amount_to_reserve: [u64; 10], // 8 * 10 bytes

    pub net_option_value: [i64; 10], // 8 * 10 bytes

    /// user's trading markets
    pub trading_markets: Vec<Pubkey>, //  36 * 32 bytes

    /// flags for liquidation use
    pub liquidate_flags: Vec<bool>, //  36 * 1 bytes

    pub client_order_id_counter: u64, // 8 bytes

    /// user's equity for on each amm
    pub amm_equities: [UserAmmEquity; 20], // 16 * 20 bytes

    /// the accumulated fee which user has paid (f_to_u)
    pub fee: u64, // 8 bytes

    /// the accumulated trading volume in USDC (f_to_u)
    pub trading_value: u64, // 8 bytes

    /// user's total deposit
    pub total_deposit: u64, // 8 bytes

    /// the delegatee of this account.
    pub delegatee: Option<Pubkey>, // 32+1 bytes

    pub fee_account: Pubkey, // 32 bytes
}

#[derive(Clone, Copy, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct UserPosition {
    pub instrument: Pubkey,
    pub long_qty: u64,  // float in 10^6
    pub short_qty: u64, // float in 10^6
}

impl UserAccount {
    /// calc the space to be allocated
    pub fn get_space_allocation() -> usize {
        7300 // 96+16+1+36*48+1+1+80*2+36*32+36
    }
}

/// Account state.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub enum AccountState {
    /// Account is not yet initialized
    Uninitialized,
    /// Account is initialized; the account owner and/or delegate may perform permitted operations
    /// on this account
    Initialized,
    /// Account has been frozen by the mint freeze authority. Neither the account owner nor
    /// the delegate are able to perform operations on this account.
    Frozen,
}

impl Default for AccountState {
    fn default() -> Self {
        AccountState::Uninitialized
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct TempPnL {
    pub amount: i64,
    pub epoch: u64,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
pub struct UserAmmEquity {
    // notioanl withdrawable usdc tokens
    pub notioanl_withdrawable: u64,
    // total lp token amount in withdraw queue
    pub lp_amount_in_queue: u64,
}
