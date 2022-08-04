use crate::prelude::*;

/// Initialize an open orders account for the amm to place orders on this optifi market
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitAMMOnOptifiMarket<'info> {
    pub optifi_exchange: AccountInfo<'info>,
    /// the amm to init
    // #[account(constraint = amm.optifi_exchange == optifi_exchange.key())]
    // pub amm: Account<'info, AMM>,
    #[account(constraint = get_amm_liquidity_auth_pda(optifi_exchange.key, program_id).0 == amm_authority.key())]
    pub amm_authority: AccountInfo<'info>,
    /// The account to use for placing orders on the DEX
    #[account(init,
            seeds = [
                PREFIX_SERUM_OPEN_ORDERS.as_bytes(),
                optifi_exchange.key().as_ref(),
                serum_market.key().as_ref(),
                amm_authority.key().as_ref(),
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
    pub rent: Sysvar<'info, Rent>,
}
