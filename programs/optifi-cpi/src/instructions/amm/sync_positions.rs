use crate::prelude::*;

#[derive(Accounts)]
#[instruction(instrument_index: u16)]
pub struct SyncPositions<'info> {
    /// optifi exchange
    pub optifi_exchange: AccountInfo<'info>,
    /// amm account
    #[account(mut, constraint = amm.optifi_exchange == optifi_exchange.key())]
    pub amm: Box<Account<'info, AmmAccount>>,
    /// the optifi market where the instrumnet to be synced is listed
    #[account(constraint = amm.trading_instruments[instrument_index as usize] == optifi_market.instrument)]
    pub optifi_market: Box<Account<'info, OptifiMarket>>,

    /// amm's base token vault (Long position)
    #[account(constraint = optifi_market.instrument_long_spl_token == accessor::mint(&long_token_vault)?)]
    pub long_token_vault: AccountInfo<'info>,

    /// amm's base token vault (Short position)
    #[account(constraint = optifi_market.instrument_short_spl_token == accessor::mint(&short_token_vault)?)]
    pub short_token_vault: AccountInfo<'info>,

    /// the serum market(orderbook)
    #[account(constraint = optifi_market.serum_market == serum_market.key())]
    pub serum_market: AccountInfo<'info>,

    /// the open orders account
    pub open_orders_account: AccountInfo<'info>,

    pub open_orders_owner: AccountInfo<'info>,
}
