use crate::prelude::*;

#[derive(Accounts)]
pub struct WithdrawFromAMM<'info> {
    /// optifi exchange
    #[account(has_one = usdc_fee_pool @ OptifiErrorCode::WrongFeeAccount)]
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    /// exchange usdc fee pool for collecting amm withdraw fees
    #[account(mut)]
    pub usdc_fee_pool: AccountInfo<'info>,

    /// the amm from which user will withdraw funds
    #[account(mut, constraint = amm.optifi_exchange == optifi_exchange.key())]
    pub amm: Box<Account<'info, AmmAccount>>,

    ///  The quote token vault of the amm
    #[account(mut, constraint = amm_quote_token_vault.key == &amm.quote_token_vault )]
    pub amm_quote_token_vault: AccountInfo<'info>,

    /// user's quote token vault from which user will transfer funds
    #[account(mut)]
    pub user_quote_token_vault: AccountInfo<'info>,

    /// amm's lp token mint address
    #[account(mut, constraint = lp_token_mint.mint_authority.unwrap() == get_amm_liquidity_auth_pda(&optifi_exchange.key(), program_id).0)]
    pub lp_token_mint: Account<'info, Mint>,

    /// amm's lp token mint authority, and usdc vault authority
    pub amm_liquidity_auth: AccountInfo<'info>,

    /// user's token vault for receiving lp tokens
    #[account(mut)]
    pub user_lp_token_vault: AccountInfo<'info>,

    /// The user account that owns the deposits
    #[account(mut, 
        has_one = owner,
        has_one = optifi_exchange
    )]
    pub user_account: Box<Account<'info, UserAccount>>,

    /// The user that owns the user account
    #[account(signer)]
    pub owner: AccountInfo<'info>,

    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
}