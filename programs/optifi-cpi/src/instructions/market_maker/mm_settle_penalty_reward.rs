use crate::prelude::*;

#[derive(Accounts)]
pub struct MmSettlePenaltyRewardContext<'info> {
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    #[account(mut,
        constraint = user_account.optifi_exchange == optifi_exchange.key(),
        constraint = user_account.is_market_maker @ OptifiErrorCode::UserIsMarketMaker,   
    )]
    pub user_account: Box<Account<'info, UserAccount>>,

    #[account(mut,
        constraint = user_account.user_margin_account_usdc == user_margin_account.key() @ OptifiErrorCode::InvalidMarginAccount,
    )]
    pub user_margin_account: AccountInfo<'info>,

    #[account(mut,
        constraint = market_maker_account.user_account == user_account.key()
    )]
    pub market_maker_account: Box<Account<'info, MarketMakerAccount>>,

    #[account(
        mut,
        constraint = usdc_fee_pool.key() == optifi_exchange.usdc_fee_pool @ OptifiErrorCode::WrongFeeAccount,
    )]
    pub usdc_fee_pool: AccountInfo<'info>,

    pub central_usdc_pool_auth: AccountInfo<'info>,

    pub token_program: AccountInfo<'info>,
}
