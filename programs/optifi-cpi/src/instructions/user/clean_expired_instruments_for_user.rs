use crate::prelude::*;

#[derive(Accounts)]
pub struct CleanInstrumentForUser<'info> {
    #[account(mut, constraint = user_account.optifi_exchange == optifi_exchange.key())]
    pub user_account: Box<Account<'info, UserAccount>>,

    pub optifi_exchange: Box<Account<'info, Exchange>>,
}
