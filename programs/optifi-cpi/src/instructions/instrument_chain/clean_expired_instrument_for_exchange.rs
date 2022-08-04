use crate::prelude::*;

#[derive(Accounts)]
pub struct CleanInstrument<'info> {
    #[account(mut)]
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    /// Clock to get the timestamp
    pub clock: Sysvar<'info, Clock>,
}
