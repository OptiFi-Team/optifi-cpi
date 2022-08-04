use crate::prelude::*;

#[derive(Accounts)]
pub struct AddAmmWithdrawReqeust<'info> {
    /// the amm from which user will withdraw funds
    #[account(mut, has_one = withdraw_queue)]
    pub amm: Box<Account<'info, AmmAccount>>,

    /// amm withdraw request queue
    #[account(mut)]
    pub withdraw_queue: AccountLoader<'info, AmmWithdrawRequestQueue>,

    /// CHECK: user's lp token vault
    #[account(
        constraint = accessor::mint(&user_lp_token_vault)? == amm.lp_token_mint,
        constraint = accessor::authority(&user_lp_token_vault)? == user_account.key()
    )]
    pub user_lp_token_vault: AccountInfo<'info>,

    /// The user account that owns the deposits
    #[account(mut, 
        has_one = owner,
        constraint = user_account.optifi_exchange == amm.optifi_exchange
    )]
    pub user_account: Box<Account<'info, UserAccount>>,

    /// The user that owns the user account
    pub owner: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
}