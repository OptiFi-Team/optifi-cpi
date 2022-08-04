use crate::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ChainData {
    /// underlying asset
    pub asset: u8, // 1 bytes
    /// option or future
    pub instrument_type: u8, // 1 bytes
    /// expiry date of the instrument, unix timestamp
    pub expiry_date: u64, // 8 bytes
    /// Duration type
    pub duration: u8, // 1 bytes
    /// Start date, as a unix timestamp
    pub start: u64, // 8 bytes
    /// Is this a perpetual contract? Only valid for futures
    pub expiry_type: u8, // 1 byte
    /// contract size percentage: 1 means actually 0.01
    pub contract_size: u64,
    pub instrument_idx: u8,
}

pub fn chain_data_to_seed_string(data: &ChainData) -> String {
    let str = data.asset.to_string()
        + data.instrument_type.to_string().as_str()
        + data.expiry_type.to_string().as_str()
        + data.expiry_date.to_string().as_str()
        + data.instrument_idx.to_string().as_str();
    msg!("Asset is {}, instrument type is {}, expiry type is {}, idx is {}, expiry date str is {}, seed str is {}",
    data.asset, data.instrument_type, data.expiry_type, data.instrument_idx, data.expiry_date.to_string(), str);
    str
}

#[derive(Accounts)]
#[instruction(bump: u8, data: ChainData)]
pub struct CreateInstrument<'info> {
    #[account(mut)]
    pub optifi_exchange: Box<Account<'info, Exchange>>,
    #[account(init,
    seeds=[PREFIX_INSTRUMENT.as_bytes(),
    optifi_exchange.key().as_mut(),
    chain_data_to_seed_string(&data).as_bytes(),
    ], payer=payer, bump, space=Chain::get_space_allocation())]
    pub instrument: Box<Account<'info, Chain>>,

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
