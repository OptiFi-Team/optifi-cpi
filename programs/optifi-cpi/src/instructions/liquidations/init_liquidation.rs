use crate::prelude::*;

#[derive(Accounts)]
pub struct InitializeLiquidation<'info> {
    pub optifi_exchange: AccountInfo<'info>,

    #[account(mut, constraint= user_account.user_margin_account_usdc == user_margin_account_usdc.key())]
    pub user_account: Box<Account<'info, UserAccount>>,

    /// user's margin account whose authority is user account(pda)
    pub user_margin_account_usdc: AccountInfo<'info>,

    #[account(mut, constraint = liquidation_state.user_account == user_account.key())]
    pub liquidation_state: Account<'info, LiquidationState>,
}
