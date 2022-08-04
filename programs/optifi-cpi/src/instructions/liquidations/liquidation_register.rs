use crate::prelude::*;

#[derive(Accounts)]
pub struct RegisterLiquidationMarket<'info> {
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    #[account(constraint = margin_stress_account.optifi_exchange == optifi_exchange.key())]
    pub margin_stress_account: Box<Account<'info, MarginStressAccount>>,

    #[account(mut, constraint = user_account.is_in_liquidation @ OptifiErrorCode::UserNotInLiquidation)]
    pub user_account: Box<Account<'info, UserAccount>>,

    #[account(mut)]
    pub user_margin_account: AccountInfo<'info>,

    #[account(
        mut,
        constraint = liquidation_state.user_account == user_account.key() @ OptifiErrorCode::InvalidAccount,
        constraint = liquidation_state.status == LiquidationStatus::CancelOrder @ OptifiErrorCode::UserNotInCancelOrder
    )]
    pub liquidation_state: Account<'info, LiquidationState>,

    pub market: Box<Account<'info, OptifiMarket>>,

    #[account(mut, constraint = serum_market.key() == market.serum_market @ OptifiErrorCode::InvalidPDA)]
    pub serum_market: AccountInfo<'info>,
    pub serum_dex_program_id: AccountInfo<'info>,
    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,

    #[account(mut,
        // constraint = open_orders.owner == &user_account.key()
    )]
    pub open_orders: AccountInfo<'info>,
    // pub open_orders_owner: AccountInfo<'info>,
    pub prune_authority: AccountInfo<'info>,
    #[account(mut)]
    pub coin_vault: AccountInfo<'info>,
    #[account(mut)]
    pub pc_vault: AccountInfo<'info>,
    /// CHECK: use in crank function
    pub vault_signer: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,

    #[account(mut)]
    pub instrument_long_spl_token_mint: AccountInfo<'info>,
    #[account(mut)]
    pub instrument_short_spl_token_mint: AccountInfo<'info>,
    #[account(
        mut,
        constraint = market.instrument_long_spl_token == accessor::mint(&user_instrument_long_token_vault)? @ OptifiErrorCode::IncorrectCoinMint,
        // constraint = user_instrument_long_token_vault.owner == &user_account.key() @ OptifiErrorCode::InvalidPDA
    )]
    pub user_instrument_long_token_vault: AccountInfo<'info>,

    #[account(
        mut,
        constraint = market.instrument_short_spl_token == accessor::mint(&user_instrument_short_token_vault)? @ OptifiErrorCode::IncorrectCoinMint,
        // constraint = user_instrument_short_token_vault.owner == &user_account.key() @ OptifiErrorCode::InvalidPDA
    )]
    pub user_instrument_short_token_vault: AccountInfo<'info>,
}
