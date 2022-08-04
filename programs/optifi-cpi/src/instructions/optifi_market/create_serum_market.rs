use crate::prelude::*;

#[derive(Accounts)]
#[instruction(bump:u8)]
pub struct InitializeSerumMarket<'info> {
    /// 0. `[writable]` the market to initialize
    /// 1. `[writable]` zeroed out request queue
    /// 2. `[writable]` zeroed out event queue
    /// 3. `[writable]` zeroed out bids
    /// 4. `[writable]` zeroed out asks
    /// 5. `[writable]` spl-token account for the coin currency
    /// 6. `[writable]` spl-token account for the price currency
    /// 7. `[]` coin currency Mint
    /// 8. `[]` price currency Mint
    /// 9. `[]` the rent sysvar
    /// 10. `[]` open orders market authority (optional)
    /// 11. `[]` prune authority (optional, requires open orders market authority)
    /// 12. `[]` crank authority (optional, requires prune authority)
    pub optifi_exchange: AccountInfo<'info>,
    #[account(mut)]
    pub market: AccountInfo<'info>,
    pub coin_mint_pk: AccountInfo<'info>,
    pub pc_mint_pk: AccountInfo<'info>,
    #[account(mut)]
    pub coin_vault_pk: AccountInfo<'info>,
    #[account(mut)]
    pub pc_vault_pk: AccountInfo<'info>,
    #[account(mut, signer)]
    pub bids_pk: AccountInfo<'info>,
    #[account(mut, signer)]
    pub asks_pk: AccountInfo<'info>,
    #[account(mut, signer)]
    pub req_q_pk: AccountInfo<'info>,
    #[account(mut, signer)]
    pub event_q_pk: AccountInfo<'info>,
    pub serum_market_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub serum_dex_program: AccountInfo<'info>,
}
