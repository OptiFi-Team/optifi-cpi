use crate::prelude::*;

#[derive(Accounts, Clone)]
pub struct MarginContext<'info> {
    /// optifi_exchange account
    pub optifi_exchange: Box<Account<'info, Exchange>>,
    /// margin stress account
    #[account(constraint = margin_stress_account.optifi_exchange == optifi_exchange.key())]
    pub margin_stress_account: Box<Account<'info, MarginStressAccount>>,
    /// user's optifi account
    #[account(mut, constraint = user_account.optifi_exchange == optifi_exchange.key())]
    pub user_account: Box<Account<'info, UserAccount>>,

    /// Clock to get the timestamp
    pub clock: Sysvar<'info, Clock>,
}
