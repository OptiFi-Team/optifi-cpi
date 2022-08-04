use crate::prelude::*;

#[derive(Accounts)]
pub struct SetDelegation<'info> {
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    /// user account - also the pda that controls the user's spl token accounts
    #[account( 
        mut,
        constraint= user_account.owner == user.key() @ OptifiErrorCode::UnauthorizedAccount,
        has_one = optifi_exchange,
    )]
    pub user_account: Box<Account<'info, UserAccount>>,

    /// The owner of user account
    #[account(signer)]
    pub user: AccountInfo<'info>,
}