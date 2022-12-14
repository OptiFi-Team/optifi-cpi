use crate::prelude::*;

/// Accounts used for mm to cancel orders on the DEX
#[derive(Accounts, Clone)]
pub struct MMCancelOrderContext<'info> {
    /// optifi_exchange account
    pub optifi_exchange: Box<Account<'info, Exchange>>,
    /// the user's wallet
    #[account(signer)]
    pub user: AccountInfo<'info>,
    /// user's optifi account
    #[account(mut,
        has_one = optifi_exchange,
        constraint = user_account.owner == user.key(),
        constraint = user_account.is_market_maker @ OptifiErrorCode::UserIsMarketMaker,
    )]
    pub user_account: Box<Account<'info, UserAccount>>,
    #[account(mut,constraint = market_maker_account.user_account == user_account.key())]
    pub market_maker_account: Box<Account<'info, MarketMakerAccount>>,
    /// user's margin account which is controlled by a pda
    #[account(mut, constraint = user_account.user_margin_account_usdc == user_margin_account.key() @ OptifiErrorCode::InvalidMarginAccount)]
    pub user_margin_account: AccountInfo<'info>,
    /// user's instrument long spl token account which is controlled by a the user's user account(pda)
    /// it stands for how many contracts the user sold for the instrument
    /// and it should be the same as order_payer_token_account if the order is ask order
    #[account(mut, constraint = accessor::authority(&user_instrument_long_token_vault)? == user_account.key() @ OptifiErrorCode::UnauthorizedTokenVault)]
    pub user_instrument_long_token_vault: AccountInfo<'info>,
    /// user's instrument short spl token account which is controlled by a the user's user account(pda)
    /// it stands for how many contracts the user bought for the instrument
    #[account(mut, constraint = accessor::authority(&user_instrument_short_token_vault)? == user_account.key() @ OptifiErrorCode::UnauthorizedTokenVault)]
    pub user_instrument_short_token_vault: AccountInfo<'info>,
    /// optifi market that binds an instrument with a serum market(orderbook)
    /// it's also the mint authority of the instrument spl token
    pub optifi_market: Box<Account<'info, OptifiMarket>>,
    /// the serum market(orderbook)
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// the user's open orders account
    #[account(mut)]
    pub open_orders: AccountInfo<'info>,
    // pub open_orders_owner: AccountInfo<'info>,
    #[account(mut)]
    pub request_queue: AccountInfo<'info>,
    #[account(mut)]
    pub event_queue: AccountInfo<'info>,
    #[account(mut)]
    pub bids: AccountInfo<'info>,
    #[account(mut)]
    pub asks: AccountInfo<'info>,
    /// The token mint address of "base" currency, aka the instrument long spl token
    #[account(mut)]
    pub coin_mint: AccountInfo<'info>,
    /// The vault for the "base" currency
    #[account(mut)]
    pub coin_vault: AccountInfo<'info>,
    /// The vault for the "quote" currency
    #[account(mut)]
    pub pc_vault: AccountInfo<'info>,
    /// serum market vault owner (pda)
    pub vault_signer: AccountInfo<'info>,
    #[account(mut)]
    pub instrument_short_spl_token_mint: AccountInfo<'info>,
    pub prune_authority: AccountInfo<'info>,
    pub serum_dex_program_id: AccountInfo<'info>,
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
}
