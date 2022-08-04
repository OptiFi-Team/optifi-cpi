use crate::prelude::*;
use anchor_lang::solana_program::program_option::COption;

fn verify_mint_authority(
    exchange: &Account<'_, Exchange>,
    mint: &Account<'_, Mint>,
    program_id: &Pubkey,
) -> bool {
    let mint_auth_pda: COption<Pubkey> =
        COption::Some(get_optifi_market_mint_auth_pda(&exchange.key(), program_id).0);
    msg!(
        "Mint authority is {}, should be {}",
        mint.mint_authority.unwrap(),
        mint_auth_pda.unwrap()
    );
    mint.mint_authority == mint_auth_pda
}

/// Create an OptiFi market and list an instrument on the market,
/// and add the market to exchange market list
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct CreateOptifiMarket<'info> {
    /// The optifi market to be created
    #[account(init,
    seeds=[PREFIX_OPTIFI_MARKET.as_bytes(),
    exchange.clone().key().as_ref(),
    &exchange.markets.len().checked_add(1).ok_or(OptifiErrorCode::NumericalOverflowError)?.to_be_bytes(),
    ], payer=payer, bump, space=OptifiMarket::get_space_allocation())]
    pub optifi_market: Box<Account<'info, OptifiMarket>>,
    /// OptiFi Exchange account
    #[account(mut)]
    pub exchange: Box<Account<'info, Exchange>>,
    /// the serum market on which the instrument will be listed
    pub serum_market: AccountInfo<'info>,
    /// The instrument to be listed
    #[account(mut, constraint = !instrument.is_listed_on_market && instrument.expiry_date as i64 > clock.unix_timestamp)]
    pub instrument: Box<Account<'info, Chain>>,
    /// the mint address of spl token for buyers of the instrument,
    /// it should be the base currency for the serum orderbook
    /// it's mint authority should be this optifi_market_mint_auth pda
    #[account(constraint = verify_mint_authority(&exchange, &long_spl_token_mint, program_id))]
    pub long_spl_token_mint: Account<'info, Mint>,
    /// the mint address of spl token for sellers of the instrument,
    /// it's mint authority should be this optifi_market_mint_auth pda
    #[account(constraint = verify_mint_authority(&exchange, &short_spl_token_mint, program_id))]
    pub short_spl_token_mint: Account<'info, Mint>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

/// Update a stopped OptiFi market with a new instrument which will be listed on it
#[derive(Accounts)]
#[instruction()]
pub struct UpdateOptifiMarket<'info> {
    /// OptiFi Exchange account
    #[account(mut)]
    pub exchange: Box<Account<'info, Exchange>>,
    /// The optifi market to be updated
    #[account(mut, constraint = optifi_market.is_stopped)]
    pub optifi_market: Box<Account<'info, OptifiMarket>>,
    /// The instrument to be listed
    #[account(mut, constraint = !instrument.is_listed_on_market && instrument.expiry_date as i64 > clock.unix_timestamp)]
    pub instrument: Box<Account<'info, Chain>>,
    /// Long token mint of the optifi_market
    #[account(constraint = instrument_long_spl_token.key() == optifi_market.instrument_long_spl_token)]
    pub instrument_long_spl_token: Box<Account<'info, Mint>>,
    pub clock: Sysvar<'info, Clock>,
}

/// Stop an OptiFi market which is expired
#[derive(Accounts)]
#[instruction()]
pub struct StopOptifiMarket<'info> {
    /// OptiFi Exchange account
    #[account(mut)]
    pub optifi_exchange: Box<Account<'info, Exchange>>,
    /// The optifi market to be updated
    #[account(mut, 
        constraint = !optifi_market.is_stopped,
        has_one = instrument_long_spl_token,
        has_one = instrument_short_spl_token,
        has_one = instrument
    )]
    pub optifi_market: Box<Account<'info, OptifiMarket>>,
    /// The instrument listed on market
    #[account(mut, constraint =  instrument.expiry_date as i64 <= clock.unix_timestamp)]
    pub instrument: Box<Account<'info, Chain>>,
    /// instrumentlong token mint address
    #[account()]
    pub instrument_long_spl_token: Account<'info, Mint>,
    /// instrument short token mint address
    #[account()]
    pub instrument_short_spl_token: Account<'info, Mint>,
    pub clock: Sysvar<'info, Clock>,
}
