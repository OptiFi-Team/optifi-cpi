use crate::prelude::*;

#[derive(Accounts)]
#[instruction(bump:u8, data: InitializeExchangeData)]
pub struct InitializeOptiFiExchange<'info> {
    /// optifi exchange account
    #[account(init,
         seeds=[PREFIX_OPTIFI_EXCHANGE.as_bytes(),
         data.uuid.as_bytes(),
         ], payer=payer, bump, space=Exchange::get_space_allocation())]
    pub optifi_exchange: Box<Account<'info, Exchange>>,
    /// optifi exchange's authority
    pub authority: AccountInfo<'info>,
    /// usdc central pool for fund settlement, its authority should be the central_usdc_pool_auth_pda
    #[account(constraint =  accessor::mint(&usdc_central_pool)? == data.usdc_mint 
        && accessor::authority(&usdc_central_pool)?== get_central_usdc_pool_auth_pda(&optifi_exchange.key(), program_id).0) ]
    pub usdc_central_pool: AccountInfo<'info>,
    /// usdc fee pool for trade fee collection, its authority should be the central_usdc_pool_auth_pda
    #[account(constraint =  accessor::mint(&usdc_fee_pool)? == data.usdc_mint
    && accessor::authority(&usdc_fee_pool)?== get_central_usdc_pool_auth_pda(&optifi_exchange.key(), program_id).0) ]
    pub usdc_fee_pool: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Default, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeExchangeData {
    /// id of the OptiFi Exchange
    pub uuid: String,
    /// OptiFi Exchange version
    pub version: u32,
    /// the authority address
    pub exchange_authority: Pubkey,
    /// the recognized usdc token mint
    pub usdc_mint: Pubkey,
    /// trusted oracle account for BTC sopt price
    pub btc_spot_oracle: Pubkey,
    /// trusted oracle account for ETH sopt price
    pub eth_spot_oracle: Pubkey,
    /// trusted oracle account for USDC sopt price
    pub usdc_spot_oracle: Pubkey,
    /// trusted oracle account for BTC IV
    pub btc_iv_oracle: Pubkey,
    /// trusted oracle account for ETH IV
    pub eth_iv_oracle: Pubkey,
    /// pubkey of og nft mint. None means og nft mode is turned off
    pub og_nft_mint: Option<Pubkey>,
    /// user deposit limit
    pub user_deposit_limit: Option<u64>
}