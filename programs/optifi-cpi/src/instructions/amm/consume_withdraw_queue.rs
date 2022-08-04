use crate::prelude::*;

#[derive(Accounts)]
pub struct ConsumeAmmWithdrawReqeust<'info> {
    /// optifi exchange
    #[account(has_one = usdc_fee_pool @ OptifiErrorCode::WrongFeeAccount)]
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    /// exchange usdc fee pool for collecting amm withdraw fees
    #[account(mut)]
    pub usdc_fee_pool: AccountInfo<'info>,

    /// the amm from which user will withdraw funds
    #[account(mut, has_one = withdraw_queue, has_one = optifi_exchange)]
    pub amm: Box<Account<'info, AmmAccount>>,

    /// margin stress account
    #[account(constraint = margin_stress_account.asset == Asset::try_from(amm.asset).unwrap())]
    pub margin_stress_account: Box<Account<'info, MarginStressAccount>>,
    /// amm withdraw request queue
    #[account(mut)]
    pub withdraw_queue: AccountLoader<'info, AmmWithdrawRequestQueue>,

    ///  The quote token vault of the amm
    #[account(mut, constraint = amm_quote_token_vault.key == &amm.quote_token_vault )]
    pub amm_quote_token_vault: AccountInfo<'info>,

    /// user's quote token vault from which user will transfer funds
    #[account(mut, constraint = user_quote_token_vault.owner == user_account.owner @ OptifiErrorCode::InvalidAccount)]
    pub user_quote_token_vault: Account<'info, TokenAccount>,

    /// amm's lp token mint address
    #[account(mut, constraint = lp_token_mint.mint_authority.unwrap() == get_amm_liquidity_auth_pda(&amm.optifi_exchange, program_id).0)]
    pub lp_token_mint: Account<'info, Mint>,

    /// amm's lp token mint authority, and usdc vault authority
    pub amm_liquidity_auth: AccountInfo<'info>,

    /// user's lp token vault
    #[account(mut)]
    pub user_lp_token_vault: AccountInfo<'info>,

    /// The user account that owns the deposits
    #[account(mut, has_one = optifi_exchange)]
    pub user_account: Box<Account<'info, UserAccount>>,

    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
}
