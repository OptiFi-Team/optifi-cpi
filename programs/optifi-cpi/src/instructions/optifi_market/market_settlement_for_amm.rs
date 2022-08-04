use crate::prelude::*;

/// Record a user's profit and loss when an instrument get expired
#[derive(Accounts)]
#[instruction()]
pub struct RecordPnLForAmm<'info> {
    /// optifi exchange account
    #[account(mut)]
    pub optifi_exchange: Box<Account<'info, Exchange>>,
    /// amm account to record pnl for
    #[account(mut, has_one = optifi_exchange)]
    pub amm_account: Box<Account<'info, AmmAccount>>,
    /// The optifi market to be settled
    #[account(mut, constraint = !optifi_market.is_stopped @OptifiErrorCode::CannotRecordPnLForStoppedMarket, has_one=serum_market, has_one = instrument)]
    pub optifi_market: Box<Account<'info, OptifiMarket>>,
    /// the serum market which the optifi market is using
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// the amm's serum open orders account
    #[account(mut)]
    pub amm_serum_open_orders: AccountInfo<'info>,
    /// The expired instrument
    #[account(
        constraint = instrument.is_listed_on_market @OptifiErrorCode::InstrumetIsDelisted,
        constraint = instrument.expiry_date as i64 <= clock.unix_timestamp @OptifiErrorCode::CannotRecordPnLBeforeMarketsExpired
    )]
    pub instrument: Box<Account<'info, Chain>>,
    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub coin_vault: AccountInfo<'info>,
    /// The vault for the "quote" currency
    #[account(mut)]
    pub pc_vault: AccountInfo<'info>,
    /// serum market vault owner (pda)
    #[account(mut)]
    pub vault_signer: AccountInfo<'info>,
    #[account(mut, constraint = amm_usdc_vault.key() == amm_account.quote_token_vault)]
    pub amm_usdc_vault: AccountInfo<'info>,
    #[account(mut)]
    pub instrument_long_spl_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub instrument_short_spl_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub amm_instrument_long_token_vault: AccountInfo<'info>,
    #[account(mut)]
    pub amm_instrument_short_token_vault: AccountInfo<'info>,
    pub prune_authority: AccountInfo<'info>,
    pub serum_dex_program_id: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
    // oracle account for spot price of the instrument's underlying asset
    pub asset_spot_price_oracle_feed: AccountInfo<'info>,
    // oracle account for usdc spot price
    pub usdc_spot_price_oracle_feed: AccountInfo<'info>,
    /// the authority of amm's amm_usdc_vault
    pub amm_liquidity_auth: AccountInfo<'info>,
}

/// Settle fund for a amm on all optifi markets that have same expiry date - simultaneous fund settlement
#[derive(Accounts)]
#[instruction()]
pub struct SettleMarketFundForAmm<'info> {
    /// optifi exchange account
    pub optifi_exchange: Box<Account<'info, Exchange>>,
    /// amm account to settle
    #[account(mut, has_one = optifi_exchange, constraint = amm_account.temp_pnl.amount != 0)]
    pub amm_account: Box<Account<'info, AmmAccount>>,
    /// usdc vault of amm account
    #[account(
        mut,
        constraint = amm_usdc_vault.key() == amm_account.quote_token_vault &&
        accessor::mint(&central_usdc_pool).unwrap() == usdc_mint.key(),
    )]
    pub amm_usdc_vault: AccountInfo<'info>,
    /// the authority of amm's amm_usdc_vault
    pub amm_liquidity_auth: AccountInfo<'info>,
    /// a central fund pool for fund settlemnet purpose
    #[account(
        mut,
        constraint = central_usdc_pool.key() == optifi_exchange.usdc_central_pool &&
        accessor::mint(&central_usdc_pool).unwrap() == usdc_mint.key(),
    )]
    pub central_usdc_pool: AccountInfo<'info>,
    pub central_usdc_pool_auth: AccountInfo<'info>,
    pub usdc_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    // =================== !!! IMPORTANT NOTE !!! =========================
    // pass all the accounts of those optifi_market with same exipry date
    // into ctx.remaining_accounts
    // ====================================================================
}
