use crate::prelude::*;

#[derive(Accounts)]
pub struct RemoveOptiFiMarketForAMM<'info> {
    pub optifi_exchange: Box<Account<'info, Exchange>>,
    /// the amm
    #[account(mut, constraint = amm.optifi_exchange == optifi_exchange.key())]
    pub amm: Box<Account<'info, AmmAccount>>,
    /// the instrumnet to remove from amm's trading instrument list
    #[account(constraint = instrument.expiry_date as i64 <= clock.unix_timestamp)]
    pub instrument: Box<Account<'info, Chain>>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct AddOptiFiMarketForAMM<'info> {
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    /// the amm
    #[account(mut)] // TODO: remove hardcoded data space
    pub amm: Box<Account<'info, AmmAccount>>,
    /// the optifi_market which list the instrument
    //#[account(constraint = optifi_market.instrument == instrument.key())]
    pub optifi_market: Box<Account<'info, OptifiMarket>>,
    /// the instrumnet to add into amm's trading instrument list, it must not be expired
    #[account(constraint = instrument.asset == amm.asset @OptifiErrorCode::MismatchedAsset,
        constraint =  instrument.expiry_date as i64 > clock.unix_timestamp @OptifiErrorCode::InstrumentExpired,
        constraint = instrument.duration == Duration::try_from(amm.duration).unwrap() @OptifiErrorCode::MismatchedDuration
        // && instrument.contract_size == amm.contract_size
    )]
    pub instrument: Box<Account<'info, Chain>>,
    #[account(constraint = optifi_market.instrument_long_spl_token == accessor::mint(&amm_long_token_vault)?)]
    pub amm_long_token_vault: AccountInfo<'info>,
    #[account(constraint = optifi_market.instrument_short_spl_token == accessor::mint(&amm_short_token_vault)?)]
    pub amm_short_token_vault: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
}
