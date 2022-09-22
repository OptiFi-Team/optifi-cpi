use crate::prelude::*;

#[derive(Accounts, Clone)]
pub struct UpdateIv<'info> {
    #[account(
        constraint = optifi_exchange.iv_authority == signer.key()
    )]
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    #[account(
        mut,
        constraint = margin_stress_account.optifi_exchange == optifi_exchange.key()
    )]
    pub margin_stress_account: Box<Account<'info, MarginStressAccount>>,

    pub signer: Signer<'info>,
}
