use crate::prelude::*;

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct RegisterMarketMaker<'info> {
    pub optifi_exchange: AccountInfo<'info>,

    #[account(mut, 
        has_one = optifi_exchange,
        constraint = user_account.owner == user.key() @ OptifiErrorCode::UnauthorizedAccount,
        constraint = !user_account.is_in_liquidation @ OptifiErrorCode::CannotPlaceOrdersInLiquidation, 
        constraint = !user_account.is_market_maker @ OptifiErrorCode::UserIsMarketMaker,         
    )]
    pub user_account: Box<Account<'info, UserAccount>>,

    #[account(init,
        seeds=[
            PREFIX_MARKET_MAKER.as_bytes(),
            optifi_exchange.key().as_ref(),
            user_account.key().as_ref()
        ],
        payer=user,
        bump,
        space=MarketMakerAccount::get_space_allocation()
    )]
    pub market_maker_account: Box<Account<'info, MarketMakerAccount>>,

    #[account(mut, constraint = user.key() == user_account.owner)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
