use crate::prelude::*;

#[derive(Accounts)]
pub struct LiquidationToAmm<'info> {
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    #[account(constraint = margin_stress_account.optifi_exchange == optifi_exchange.key())]
    pub margin_stress_account: Box<Account<'info, MarginStressAccount>>,

    #[account(mut, constraint=user_account.is_in_liquidation)]
    pub user_account: Box<Account<'info, UserAccount>>,

    #[account(mut)]
    pub user_margin_account: AccountInfo<'info>,

    #[account(mut,
        constraint = liquidation_state.user_account == user_account.key()
    )]
    pub liquidation_state: Account<'info, LiquidationState>,

    pub user_instrument_long_token_vault: AccountInfo<'info>,

    pub user_instrument_short_token_vault: AccountInfo<'info>,

    pub optifi_market: Box<Account<'info, OptifiMarket>>,

    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,

    #[account(mut, signer)]
    pub liquidator: AccountInfo<'info>,

    /// The vault for the "quote" currency
    #[account(mut)]
    pub pc_vault: AccountInfo<'info>,
    /// the mint authoriity of both long and short spl tokens
    pub instrument_token_mint_authority_pda: AccountInfo<'info>,
    /// The token mint address of "base" currency, aka the instrument long spl token
    #[account(mut)]
    pub instrument_long_spl_token_mint: AccountInfo<'info>,
    /// the instrument short spl token
    #[account(mut)]
    pub instrument_short_spl_token_mint: AccountInfo<'info>,
    #[account(mut)]
    pub amm_instrument_long_token_vault: AccountInfo<'info>,
    /// amm's instrument short spl token account
    #[account(mut)]
    pub amm_instrument_short_token_vault: AccountInfo<'info>,

    pub amm_liquidity_auth: AccountInfo<'info>,

    pub amm: Box<Account<'info, AmmAccount>>,
}
