use crate::prelude::*;

#[derive(Accounts)]
pub struct CalculateAmmProposal<'info> {
    /// margin stress account
    pub margin_stress_account: Box<Account<'info, MarginStressAccount>>,
    /// amm account
    #[account(mut, constraint = amm.optifi_exchange ==  margin_stress_account.optifi_exchange)]
    pub amm: Box<Account<'info, AmmAccount>>,
}
