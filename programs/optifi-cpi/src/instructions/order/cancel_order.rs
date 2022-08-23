use crate::prelude::*;

/// Accounts used to place orders on the DEX
#[derive(Accounts)]
pub struct CancelOrderContext<'info> {
    /// optifi_exchange account
    pub optifi_exchange: Box<Account<'info, Exchange>>,
    #[account(
        constraint = margin_stress_account.optifi_exchange == optifi_exchange.key(),
        constraint = !margin_stress_account.is_timeout() @ OptifiErrorCode::TimeOut,
    )]
    pub margin_stress_account: Box<Account<'info, MarginStressAccount>>,
    /// the user's wallet
    pub user: Signer<'info>,
    /// user's optifi account
    #[account(mut,
        has_one = optifi_exchange,
        constraint = user_account.owner == user.key() || user_account.delegatee == Some(user.key()) @OptifiErrorCode::UnauthorizedAccount,
        constraint = !user_account.is_market_maker @ OptifiErrorCode::UserIsMarketMaker,
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    /// user's margin account which is controlled by a pda
    #[account(mut, constraint = user_account.user_margin_account_usdc == user_margin_account.key() @ OptifiErrorCode::InvalidMarginAccount)]
    pub user_margin_account: AccountInfo<'info>,
    /// the serum market(orderbook)
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// the user's open orders account
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,
    #[account(mut, constraint = usdc_fee_pool.key() == optifi_exchange.usdc_fee_pool @ OptifiErrorCode::WrongFeeAccount)]
    pub usdc_fee_pool: AccountInfo<'info>,
    pub central_usdc_pool_auth: AccountInfo<'info>,
    pub serum_dex_program_id: AccountInfo<'info>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(
        mut,
        has_one = user_account.key()
    )]
    pub fee_account: Box<Account<'info, FeeAccount>>,
}
