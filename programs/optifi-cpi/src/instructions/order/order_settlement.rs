use crate::prelude::*;

#[derive(Accounts)]
pub struct OrderSettlement<'info> {
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    #[account(mut,
        constraint = user_account.optifi_exchange == optifi_exchange.key(),
        // constraint = !user_account.is_market_maker @ OptifiErrorCode::UserIsMarketMaker,             
    )]
    pub user_account: Box<Account<'info, UserAccount>>,

    #[account(mut, constraint = !optifi_market.is_stopped)]
    pub optifi_market: Box<Account<'info, OptifiMarket>>,

    #[account(mut, constraint = serum_market.key() == optifi_market.serum_market)]
    pub serum_market: AccountInfo<'info>,

    #[account(mut)]
    pub user_serum_open_orders: AccountInfo<'info>,

    #[account(mut)]
    pub coin_vault: AccountInfo<'info>,

    #[account(mut)]
    pub pc_vault: AccountInfo<'info>,

    #[account(mut)]
    pub instrument_long_spl_token_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub instrument_short_spl_token_mint: AccountInfo<'info>,
    #[account(mut, constraint = accessor::authority(&user_instrument_long_token_vault)? == user_account.key() @ OptifiErrorCode::UnauthorizedTokenVault)]
    pub user_instrument_long_token_vault: AccountInfo<'info>,
    #[account(mut, constraint = accessor::authority(&user_instrument_short_token_vault)? == user_account.key() @ OptifiErrorCode::UnauthorizedTokenVault)]
    pub user_instrument_short_token_vault: AccountInfo<'info>,

    #[account(mut, constraint = user_account.user_margin_account_usdc == user_margin_account.key() @ OptifiErrorCode::InvalidMarginAccount)]
    pub user_margin_account: AccountInfo<'info>,

    #[account(mut)]
    pub vault_signer: AccountInfo<'info>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    pub serum_dex_program_id: AccountInfo<'info>,

    #[account(mut)]
    pub event_queue: AccountInfo<'info>,

    pub consume_events_authority: AccountInfo<'info>,

    #[account(
        mut,
        seeds=[
            FEE_ACCOUNT.as_bytes(),
            optifi_exchange.key().as_ref(),
            user_account.key().as_ref()
        ], 
        bump
    )]
    pub fee_account: Box<Account<'info, FeeAccount>>,
}