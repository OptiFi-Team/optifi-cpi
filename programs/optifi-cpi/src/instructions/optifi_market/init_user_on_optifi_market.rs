use crate::prelude::*;

/// Initialize an open orders account for the user to place orders on this optifi market
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitUserOnOptifiMarket<'info> {
    pub optifi_exchange: AccountInfo<'info>,
    pub user: AccountInfo<'info>,
    #[account(mut,
        has_one = optifi_exchange,
        constraint= user_account.owner == user.key() @ OptifiErrorCode::UnauthorizedAccount,
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    /// The account to use for placing orders on the DEX
    #[account(init,
            seeds = [
                PREFIX_SERUM_OPEN_ORDERS.as_bytes(),
                optifi_exchange.key().as_ref(),
                serum_market.key().as_ref(),
                user_account.key().as_ref(),
            ],
            bump,
            payer = payer,
            owner = serum_dex_program_id.key(),
            space = std::mem::size_of::<OpenOrders>() + 12,
            )] // TODO: figure out skip rent_exempt ???
    pub serum_open_orders: AccountInfo<'info>,
    /// The optifi market to initialize for
    #[account( constraint = !optifi_market.is_stopped)]
    pub optifi_market: Box<Account<'info, OptifiMarket>>,
    /// the serum market the optifi market is using
    pub serum_market: AccountInfo<'info>,
    /// serum market authority which is required when init open orders if it is Some()
    pub serum_market_authority: AccountInfo<'info>,
    // pub clock: Sysvar<'info, Clock>,
    pub serum_dex_program_id: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    // pub pda: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}
