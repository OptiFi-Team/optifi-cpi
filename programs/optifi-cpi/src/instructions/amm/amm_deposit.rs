use crate::prelude::*;

#[derive(Accounts)]
pub struct DepositToAMM<'info> {
    /// CHECK:
    pub optifi_exchange: AccountInfo<'info>,
    /// the amm to which user will deposits funds
    #[account(mut, constraint = amm.optifi_exchange == optifi_exchange.key())]
    pub amm: Box<Account<'info, AmmAccount>>,

    /// The quote token vault of amm - usdc vault
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
    /// in order to calc the performance fee, the authority of the lp token vault must be user account
    #[account(mut, constraint = accessor::authority(&user_lp_token_vault)? == user_account.key())]
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
