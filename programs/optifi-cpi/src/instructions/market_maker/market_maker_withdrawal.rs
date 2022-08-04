use crate::prelude::*;

#[derive(Accounts)]
pub struct ScheduleMarketMakerWithdrawal<'info> {
    pub optifi_exchange: AccountInfo<'info>,

    #[account(mut, 
        has_one = optifi_exchange,
        constraint = user_account.owner == user.key() @ OptifiErrorCode::UnauthorizedAccount,
        constraint = !user_account.is_in_liquidation @ OptifiErrorCode::CannotPlaceOrdersInLiquidation, 
        constraint = user_account.is_market_maker @ OptifiErrorCode::UserIsMarketMaker,         
    )]
    pub user_account: Box<Account<'info, UserAccount>>,

    /// user's margin account whose authority is user account(pda)
    #[account(mut, constraint = user_account.user_margin_account_usdc == user_margin_account_usdc.key() @ OptifiErrorCode::UnauthorizedTokenVault)]
    pub user_margin_account_usdc: AccountInfo<'info>,

    #[account(mut, constraint = market_maker_account.user_account == user_account.key())]
    pub market_maker_account: Box<Account<'info, MarketMakerAccount>>,

    #[account(signer)]
    pub user: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>,
}


#[derive(Accounts)]
pub struct ExecuteMarketMakerWithdrawal<'info> {
    pub optifi_exchange: AccountInfo<'info>,

    #[account(mut, 
        has_one = optifi_exchange,
        constraint = user_account.owner == user.key() @ OptifiErrorCode::UnauthorizedAccount,
        constraint = !user_account.is_in_liquidation @ OptifiErrorCode::CannotPlaceOrdersInLiquidation, 
        constraint = user_account.is_market_maker @ OptifiErrorCode::UserIsMarketMaker,         
    )]
    pub user_account: Box<Account<'info, UserAccount>>,

    /// user's margin account whose authority is user account(pda)
    #[account(mut, constraint = user_account.user_margin_account_usdc == user_margin_account_usdc.key() @ OptifiErrorCode::InvalidMarginAccount)]
    pub user_margin_account_usdc: AccountInfo<'info>,

    #[account(
        constraint = accessor::mint(&withdraw_dest).unwrap() == accessor::mint(&user_margin_account_usdc).unwrap()
    )]
    pub withdraw_dest: AccountInfo<'info>,

    #[account(mut, constraint = market_maker_account.user_account == user_account.key())]
    pub market_maker_account: Box<Account<'info, MarketMakerAccount>>,

    #[account(signer)]
    pub user: AccountInfo<'info>,

    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
}
