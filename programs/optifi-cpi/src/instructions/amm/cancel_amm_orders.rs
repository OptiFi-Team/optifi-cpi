use crate::prelude::*;

#[derive(Accounts)]
#[instruction(instrument_index: u16)]
pub struct CancelAmmOrders<'info> {
    /// optifi exchange account
    pub optifi_exchange: AccountInfo<'info>,
    /// the amm to update oders for
    #[account(mut, constraint = amm.optifi_exchange == optifi_exchange.key())]
    pub amm: Box<Account<'info, AmmAccount>>,
    /// amm's margin account(usdc vault) which is controlled by amm_authority (a pda)
    #[account(mut)]
    pub amm_usdc_vault: AccountInfo<'info>,
    /// the authority of amm's amm_usdc_vault
    pub amm_authority: AccountInfo<'info>,
    /// amm's instrument long spl token account
    #[account(mut)]
    pub amm_instrument_long_token_vault: AccountInfo<'info>,
    /// optifi market that binds an instrument with a serum market(orderbook)
    /// it's also the mint authority of the instrument spl token
    #[account(has_one = serum_market, constraint = amm.trading_instruments[instrument_index as usize] == optifi_market.instrument)]
    pub optifi_market: Box<Account<'info, OptifiMarket>>,
    /// the serum market(orderbook)
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// amm's open orders account for this optifi market,
    /// its owner is amm account(pda)
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,
    /// The vault for the "base" currency
    #[account(mut)]
    pub coin_vault: AccountInfo<'info>,
    /// The vault for the "quote" currency
    #[account(mut)]
    pub pc_vault: AccountInfo<'info>,
    /// serum market vault owner (pda)
    pub vault_signer: AccountInfo<'info>,
    /// the mint authoriity of both long and short spl tokens
    pub instrument_token_mint_authority_pda: AccountInfo<'info>,
    pub prune_authority: AccountInfo<'info>,
    pub serum_dex_program_id: AccountInfo<'info>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
}
