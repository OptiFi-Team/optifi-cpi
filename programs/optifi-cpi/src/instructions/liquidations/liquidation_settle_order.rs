use crate::prelude::*;

#[derive(Accounts)]
// #[instruction(bump: u8)]
pub struct LiquidationSettleOrder<'info> {
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    #[account(mut, constraint=user_account.is_in_liquidation)]
    pub user_account: Box<Account<'info, UserAccount>>,

    #[account(mut)]
    pub user_margin_account: AccountInfo<'info>,

    #[account(mut,
        // seeds=[
        //     PREFIX_LIQUIDATION_STATE.as_bytes(),
        //     optifi_exchange.key().as_ref(),
        //     user_account.key().as_ref()
        // ],
        // bump,
        constraint = liquidation_state.user_account == user_account.key()
    )]
    pub liquidation_state: Account<'info, LiquidationState>,

    #[account(mut)]
    pub user_instrument_long_token_vault: AccountInfo<'info>,
    #[account(mut)]
    pub user_instrument_short_token_vault: AccountInfo<'info>,

    #[account(mut)]
    pub optifi_market: Box<Account<'info, OptifiMarket>>,

    #[account(mut, constraint = serum_market.key() == optifi_market.serum_market)]
    pub serum_market: AccountInfo<'info>,
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,

    #[account(mut)]
    pub coin_vault: AccountInfo<'info>,
    #[account(mut)]
    pub pc_vault: AccountInfo<'info>,
    /// CHECK: use in crank function
    pub vault_signer: AccountInfo<'info>,
    #[account(mut)]
    pub instrument_long_spl_token_mint: AccountInfo<'info>,
    #[account(mut)]
    pub instrument_short_spl_token_mint: AccountInfo<'info>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,

    pub serum_dex_program_id: AccountInfo<'info>,

    #[account(mut, signer)]
    pub liquidator: AccountInfo<'info>,
}
