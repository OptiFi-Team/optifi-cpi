use crate::prelude::*;

#[derive(Accounts)]
#[instruction(bump: u8, data: InitializeAMMData)]
pub struct InitializeAMM<'info> {
    /// the optifi exchange
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    /// the amm account to be initialized
    #[account(init, 
        seeds=[
            PREFIX_AMM.as_bytes(),
            optifi_exchange.key().as_ref(),
            &[data.amm_idx],
        ],
        payer=payer, bump, space=AmmAccount::get_space_allocation())]
    pub amm: Box<Account<'info, AmmAccount>>,

    /// amm withdraw request queue
    #[account(zero)]
    pub withdraw_queue: AccountLoader<'info, AmmWithdrawRequestQueue>,

    /// its token mint must be the usdc recogonized by optifi exchange, and owner is amm's liquidity auth pda
    pub usdc_token_vault: AccountInfo<'info>,
    /// amm's lp token mint address, whose mint authority is amm's liquidity auth pda
    pub lp_token_mint: Account<'info, Mint>,
    /// authority of usdc_token_vault and lp_token_mint
    pub amm_liqudity_auth:  AccountInfo<'info>,

    /// The user that owns the deposits
    #[account(mut)]
    pub payer: Signer<'info>,

    // #[account(mut)]
    // pub deposit_source: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
   
    /// mango accounts for amm delta hedging on mango market
    pub mango_program: AccountInfo<'info>,
    #[account(mut)]
    pub mango_group: AccountInfo<'info>,
    /// amm's mango account on mango market
    #[account(mut)]
    pub amm_mango_account:AccountInfo<'info>,
    /// mango perp market which has the same underlying asset for amm hedging
    pub perp_market: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeAMMData {
    /// idx of the amm
    pub amm_idx: u8,
    /// amm capacity percentage (25 is actually 25%)
    pub amm_capacity: u64,
    /// bump seed used to derive this amm address
    pub bump: u8,
    /// underlying asset
    pub asset: u8,
    /// number of trading instruments
    pub num_instruments: u8,
    /// Duration type
    pub duration: u8, // 1 bytes
    /// the contract size *10^6 (f_to_u_repr)
    pub contract_size: u64,
}
