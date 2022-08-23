use crate::prelude::*;

#[derive(Accounts)]
pub struct InitializeFeeAccount<'info> {
    /// the optifi_exchange account
    pub optifi_exchange: AccountInfo<'info>,
    /// the user's optifi account to be initialized
    #[account(mut,
        has_one = optifi_exchange,
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    /// the user's liquidation account to be initialized for liquidation use
    #[account(
        init, 
        payer=payer, 
        seeds=[
            FEE_ACCOUNT.as_bytes(),
            optifi_exchange.key().as_ref(),
            user_account.key().as_ref()
        ], 
        bump,
        space=FeeAccount::get_space_allocation()
    )]
    pub fee_account: Account<'info, FeeAccount>,

    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}