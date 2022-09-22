use crate::prelude::*;

#[derive(Accounts)]
pub struct ConsumeEventQueue<'info> {
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    #[account(mut)]
    pub serum_market: AccountInfo<'info>,

    #[account(mut)]
    pub event_queue: AccountInfo<'info>,

    #[account(mut)]
    pub user_serum_open_orders: AccountInfo<'info>,

    pub consume_events_authority: AccountInfo<'info>,

    pub serum_dex_program_id: AccountInfo<'info>,
}
