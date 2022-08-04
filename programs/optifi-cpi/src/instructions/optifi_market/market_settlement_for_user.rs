use crate::prelude::*;

/// Record a user's profit and loss when an instrument get expired
#[derive(Accounts)]
#[instruction()]
pub struct RecordPnLForOneUser<'info> {
    /// optifi exchange account
    #[account(mut)]
    pub optifi_exchange: Box<Account<'info, Exchange>>,
    /// the user's optifi account
    #[account(mut, has_one = optifi_exchange, has_one = user_margin_account_usdc)]
    pub user_account: Box<Account<'info, UserAccount>>,
    /// The optifi market to be settled
    #[account(mut, constraint = !optifi_market.is_stopped @OptifiErrorCode::CannotRecordPnLForStoppedMarket, has_one=serum_market, has_one = instrument)]
    pub optifi_market: Box<Account<'info, OptifiMarket>>,
    /// the serum market which the optifi market is using
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// the user's serum open orders account
    #[account(mut)]
    pub user_serum_open_orders: AccountInfo<'info>,
    /// The expired instrument
    #[account(
        constraint = instrument.is_listed_on_market @OptifiErrorCode::InstrumetIsDelisted,
        constraint = instrument.expiry_date as i64 <= clock.unix_timestamp @OptifiErrorCode::CannotRecordPnLBeforeMarketsExpired
    )]
    pub instrument: Box<Account<'info, Chain>>,
    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub coin_vault: AccountInfo<'info>,
    /// The vault for the "quote" currency
    #[account(mut)]
    pub pc_vault: AccountInfo<'info>,
    /// serum market vault owner (pda)
    #[account(mut)]
    pub vault_signer: AccountInfo<'info>,
    #[account(mut)]
    pub user_margin_account_usdc: AccountInfo<'info>,
    #[account(mut)]
    pub instrument_long_spl_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub instrument_short_spl_token_mint: Account<'info, Mint>,
    #[account(mut, constraint = accessor::authority(&user_instrument_long_token_vault)? == user_account.key() @ OptifiErrorCode::UnauthorizedTokenVault)]
    pub user_instrument_long_token_vault: AccountInfo<'info>,
    #[account(mut, constraint = accessor::authority(&user_instrument_short_token_vault)? == user_account.key() @ OptifiErrorCode::UnauthorizedTokenVault)]
    pub user_instrument_short_token_vault: AccountInfo<'info>,
    pub prune_authority: AccountInfo<'info>,
    pub serum_dex_program_id: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    // oracle account for spot price of the instrument's underlying asset
    pub asset_spot_price_oracle_feed: AccountInfo<'info>,
    // oracle account for usdc spot price
    pub usdc_spot_price_oracle_feed: AccountInfo<'info>,
}

/// Settle fund for a user on all optifi markets that have same expiry date - simultaneous fund settlement
#[derive(Accounts)]
pub struct SettleMarketFundForOneUser<'info> {
    /// optifi exchange account
    pub optifi_exchange: Box<Account<'info, Exchange>>,
    /// The optifi market to be settled
    // #[account(mut, constraint = !optifi_market.is_stopped, has_one = instrument_long_spl_token, has_one = instrument_short_spl_token)]
    // pub optifi_market: Box<Account<'info, OptifiMarket>>,
    #[account(mut, has_one = optifi_exchange, has_one = user_margin_account_usdc, constraint = user_account.temp_pnl.amount != 0 )]
    pub user_account: Box<Account<'info, UserAccount>>,
    /// user's margin account
    #[account(mut, constraint = accessor::mint(&user_margin_account_usdc).unwrap() == usdc_mint.key())]
    pub user_margin_account_usdc: AccountInfo<'info>,
    /// a central fund pool for fund settlemnet purpose
    #[account(
        mut,
        constraint = central_usdc_pool.key() == optifi_exchange.usdc_central_pool &&
        accessor::mint(&central_usdc_pool).unwrap() == usdc_mint.key(),
    )]
    pub central_usdc_pool: AccountInfo<'info>,
    pub central_usdc_pool_auth: AccountInfo<'info>,
    pub usdc_mint: AccountInfo<'info>,
    // /// long token total supply should be 0
    // #[account(mut, constraint = instrument_long_spl_token.supply == 0 )]
    // pub instrument_long_spl_token: Account<'info, Mint>,
    // /// short token total supply should be 0
    // #[account(mut, constraint = instrument_short_spl_token.supply == 0 )]
    // pub instrument_short_spl_token: Account<'info, Mint>,
    pub token_program: AccountInfo<'info>,
    // =================== !!! IMPORTANT NOTE !!! =========================
    // pass all the accounts of those optifi_market with same exipry date
    // into ctx.remaining_accounts
    // ====================================================================
}
