use crate::prelude::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub optifi_exchange: AccountInfo<'info>,

    /// user account - also the pda that controls the user's spl token accounts
    #[account(
        has_one = optifi_exchange,
        constraint = user_account.owner == user.key() @ OptifiErrorCode::UnauthorizedAccount,
        constraint = user_account.user_margin_account_usdc == user_margin_account_usdc.key() @ OptifiErrorCode::InvalidMarginAccount ,
        constraint = !user_account.is_market_maker @ OptifiErrorCode::UserIsMarketMaker,             
    )]
    pub user_account: Box<Account<'info, UserAccount>>,

    /// user's margin account whose authority is user account(pda)
    #[account(mut)]
    pub user_margin_account_usdc: AccountInfo<'info>,

    /// The owner of user account
    #[account(signer)]
    pub user: AccountInfo<'info>,

    /// the destination token account to which funds will be withdrawed
    #[account(mut, constraint= !withdraw_dest.data_is_empty() && withdraw_dest.lamports() > 0)]
    pub withdraw_dest: AccountInfo<'info>,

    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
}
