use super::*;

#[derive(Accounts)]
pub struct UpdateFutureOrder<'info> {
    // /// optifi exchange
    // pub optifi_exchange: Box<Account<'info, Exchange>>,
    /// amm account
    #[account(mut)]
    pub amm: Box<Account<'info, AmmAccount>>,

    /// mango program
    pub mango_program: AccountInfo<'info>,
    /// user's mango account
    pub mango_group: AccountInfo<'info>,
    // pub mango_group_signer: AccountInfo<'info>,
    /// user's mango account
    #[account(mut)]
    pub mango_account: AccountInfo<'info>,
    /// owner of mango_account
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    pub mango_cache: AccountInfo<'info>,
    #[account(mut)]
    pub perp_market: AccountInfo<'info>,
    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub root_bank: AccountInfo<'info>,
    #[account(mut)]
    pub node_bank: AccountInfo<'info>,
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    #[account(mut)]
    pub owner_token_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
