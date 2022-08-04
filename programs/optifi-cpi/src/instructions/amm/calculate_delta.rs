use crate::prelude::*;

#[derive(Accounts)]
pub struct CalculateAmmDelta<'info> {
    /// margin stress account
    #[account(constraint = margin_stress_account.asset == Asset::try_from(amm.asset).unwrap())]
    pub margin_stress_account: Box<Account<'info, MarginStressAccount>>,
    /// amm account
    #[account(mut, constraint = amm.optifi_exchange == margin_stress_account.optifi_exchange)]
    pub amm: Box<Account<'info, AmmAccount>>,
    /// amm's quote token vault to get the USDC balance
    #[account(constraint = amm.quote_token_vault == quote_token_vault.key())]
    pub quote_token_vault: AccountInfo<'info>,
}
