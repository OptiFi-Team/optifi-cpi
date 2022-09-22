use crate::prelude::*;

#[derive(Accounts, Clone)]
pub struct CalculateMarginStressContext<'info> {
    /// optifi_exchange account
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    #[account(mut, constraint = margin_stress_account.optifi_exchange == optifi_exchange.key())]
    pub margin_stress_account: Box<Account<'info, MarginStressAccount>>,

    // Oracle to get the spot price
    pub asset_feed: AccountInfo<'info>,
    pub usdc_feed: AccountInfo<'info>,
}
