use crate::prelude::*;

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    /// user account - also the pda that controls the user's spl token accounts
    #[account( 
        mut,
        constraint= user_account.owner == user.key() @ OptifiErrorCode::UnauthorizedAccount,
        has_one = user_margin_account_usdc @  OptifiErrorCode::UnauthorizedTokenVault,
        has_one = optifi_exchange,
    )]
    pub user_account: Box<Account<'info, UserAccount>>,

    /// user's margin account whose authority is user account(pda)
    #[account(mut)]
    pub user_margin_account_usdc: AccountInfo<'info>,

    /// The owner of user account
    #[account(signer)]
    pub user: AccountInfo<'info>,

    /// from address - user's usdc token account with the usdc tokens to be deposited
    #[account(mut)]
    pub deposit_source: AccountInfo<'info>,

    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
}
