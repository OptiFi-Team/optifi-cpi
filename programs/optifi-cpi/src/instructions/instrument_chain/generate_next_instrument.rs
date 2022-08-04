use crate::prelude::*;

#[derive(Accounts)]
#[instruction(bump: u8, bump2: u8,data: ChainData, data2: ChainData)]
pub struct GenerateNextInstrument<'info> {
    #[account(mut)]
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    #[account(init,
    seeds=[PREFIX_INSTRUMENT.as_bytes(),
    optifi_exchange.key().as_mut(),
    chain_data_to_seed_string(&data).as_bytes(),
    ], payer=payer, bump, space=Chain::get_space_allocation())]
    pub instrument: Box<Account<'info, Chain>>,

    #[account(init,
    seeds=[PREFIX_INSTRUMENT.as_bytes(),
    optifi_exchange.key().as_mut(),
    chain_data_to_seed_string(&data2).as_bytes(),
    ], payer=payer, bump, space=Chain::get_space_allocation())]
    pub instrument2: Box<Account<'info, Chain>>,

    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    // oracle feed account for spot price of the instrument's underlying asset
    pub asset_spot_price_oracle_feed: AccountInfo<'info>,
    // oracle feed account for iv of the instrument's underlying asset
    pub asset_iv_oracle_feed: AccountInfo<'info>,
    /// Clock to get the timestamp
    pub clock: Sysvar<'info, Clock>,
}
