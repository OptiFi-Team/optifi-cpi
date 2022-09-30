use crate::prelude::*;

#[derive(Accounts)]
pub struct MmCalculationContext<'info> {
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    #[account(
        constraint = margin_stress_account.optifi_exchange == optifi_exchange.key(),
        constraint = !margin_stress_account.is_timeout() @ OptifiErrorCode::TimeOut,
    )]
    pub margin_stress_account: Box<Account<'info, MarginStressAccount>>,
    
    #[account(mut,
        constraint = user_account.optifi_exchange == optifi_exchange.key(),
        constraint = user_account.is_market_maker @ OptifiErrorCode::UserIsMarketMaker,   
    )]
    pub user_account: Box<Account<'info, UserAccount>>,

    #[account(mut)]
    pub user_margin_account: AccountInfo<'info>,

    #[account(mut,
        constraint = market_maker_account.user_account == user_account.key()
    )]
    pub market_maker_account: Box<Account<'info, MarketMakerAccount>>,

    #[account(mut, constraint = !optifi_market.is_stopped)]
    pub optifi_market: Box<Account<'info, OptifiMarket>>,

    #[account(mut, constraint = serum_market.key() == optifi_market.serum_market)]
    pub serum_market: AccountInfo<'info>,

    #[account(mut)]
    pub open_orders: AccountInfo<'info>,

    #[account(mut)]
    pub user_instrument_long_token_vault: AccountInfo<'info>,

    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,

    pub serum_dex_program_id: AccountInfo<'info>,
}