use crate::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitUserAccountBumpSeeds {
    pub user_account: u8,
    pub liquidation_account: u8,
}

#[derive(Accounts)]
#[instruction(bump: InitUserAccountBumpSeeds)]
pub struct InitializeUserAccount<'info> {
    /// the optifi_exchange account
    #[account(mut)]
    pub optifi_exchange: Box<Account<'info, Exchange>>,
    /// the user's optifi account to be initialized
    #[account(init,
        seeds=[PREFIX_USER_ACCOUNT.as_bytes(), optifi_exchange.key().as_ref(), owner.key().as_ref()],
        payer=payer,
        bump,
        space=UserAccount::get_space_allocation()
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    /// the user's liquidation account to be initialized for liquidation use
    #[account(init, 
        payer=payer, 
        seeds=[
            PREFIX_LIQUIDATION_STATE.as_bytes(),
            optifi_exchange.key().as_ref(),
            user_account.key().as_ref()
        ], 
        bump,
        space=LiquidationState::get_space_allocation()
    )]
    pub liquidation_account: Box<Account<'info, LiquidationState>>,
    /// the margin account into which user will deposits spl token
    #[account(
        mut, 
        token::mint = optifi_exchange.usdc_mint,
        token::authority = user_account,
    )]
    pub user_margin_account_usdc: Box<Account<'info, TokenAccount>>,
    /// owner of the user account
    #[account(signer, constraint= owner.data_is_empty() && owner.lamports() > 0)]
    pub owner: AccountInfo<'info>,
    /// payer to pay accounts rent fee
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}