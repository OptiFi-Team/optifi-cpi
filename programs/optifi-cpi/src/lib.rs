pub mod constants;
pub mod errors;
pub mod financial;
pub mod instructions;
pub mod state;
pub mod utils;

#[cfg(not(feature = "devnet"))]
declare_id!("optFiKjQpoQ3PvacwnFWaPUAqXCETMJSz2sz8HwPe9B");

#[cfg(feature = "devnet")]
declare_id!("DeVoPfWrDn2UTA1MbSfagvpQxA91MpNFQnhHRw19dK34");

pub mod prelude {
    pub use crate::constants::*;
    pub use crate::errors::*;
    pub use crate::financial::*;
    pub use crate::instructions::*;
    pub use crate::state::*;
    pub use crate::utils::*;

    pub use anchor_lang::prelude::*;

    pub use anchor_spl::token;
    pub use anchor_spl::token::{accessor, Burn, Mint, MintTo, Token, TokenAccount, Transfer};

    pub use solana_program::account_info::AccountInfo;
    pub use solana_program::log::sol_log_compute_units;
    pub use solana_program::program::{invoke, invoke_signed};
    pub use solana_program::program_pack::IsInitialized;
    pub use solana_program::pubkey::Pubkey;

    pub use serum_dex::state::OpenOrders;

    pub use std::convert::TryFrom;
}

use crate::prelude::*;

#[program]
pub mod optifi_cpi {
    #![allow(unused_variables)]

    use super::*;

    /// Initialize OptiFi Exchange
    pub fn initialize(
        ctx: Context<InitializeOptiFiExchange>,
        bump: u8,
        data: InitializeExchangeData,
    ) -> Result<()> {
        Ok(())
    }

    /// Create a new instrument with specified data
    pub fn create_new_instrument(
        ctx: Context<CreateInstrument>,
        bump: u8,
        data: ChainData,
    ) -> Result<()> {
        Ok(())
    }

    /// dynamically add instruments with new strikes
    pub fn generate_next_instrument(
        ctx: Context<GenerateNextInstrument>,
        bump: u8,
        bump2: u8,
        data: ChainData,
        data2: ChainData,
    ) -> Result<()> {
        Ok(())
    }

    /// Clean the expired instruments
    pub fn clean_expired_instruments(ctx: Context<CleanInstrument>) -> Result<()> {
        Ok(())
    }

    /// Initialize a new serum market(orderbook)
    pub fn initialize_serum_orderbook(
        ctx: Context<InitializeSerumMarket>,
        coin_lot_size: u64,
        pc_lot_size: u64,
        vault_signer_nonce: u64,
        pc_dust_threshold: u64,
    ) -> Result<()> {
        Ok(())
    }

    /// Create a new optifi market with an instrument listed on it
    pub fn create_optifi_market(ctx: Context<CreateOptifiMarket>, bump: u8) -> Result<()> {
        Ok(())
    }

    /// Stop an optifi market which is expired
    pub fn stop_optifi_market(ctx: Context<StopOptifiMarket>) -> Result<()> {
        Ok(())
    }

    /// Update a stopped optifi market with a new instrument
    pub fn update_optifi_market(ctx: Context<UpdateOptifiMarket>) -> Result<()> {
        Ok(())
    }

    /// Init an open orders for the user to place orders on an optifi market
    pub fn init_user_on_optifi_market(
        ctx: Context<InitUserOnOptifiMarket>,
        bump: u8,
    ) -> Result<()> {
        Ok(())
    }

    /// Init an open orders account for the amm to place orders on an optifi market
    pub fn init_amm_on_optifi_market(ctx: Context<InitAMMOnOptifiMarket>, bump: u8) -> Result<()> {
        Ok(())
    }

    /// Submit a limit order
    pub fn place_order(
        ctx: Context<PlaceOrderContext>,
        side: OrderSide,
        limit: u64,
        max_coin_qty: u64,
        max_pc_qty: u64,
        order_type: u8,
    ) -> Result<()> {
        Ok(())
    }

    /// Cancel a previously placed order
    pub fn cancel_order_by_client_order_id(
        ctx: Context<CancelOrderContext>,
        side: OrderSide,
        client_order_id: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn settle_order_funds(ctx: Context<OrderSettlement>) -> Result<()> {
        Ok(())
    }

    /// Initialize user's optifi account
    pub fn init_user_account<'info>(
        ctx: Context<'_, '_, '_, 'info, InitializeUserAccount<'info>>,
        bump: InitUserAccountBumpSeeds,
    ) -> Result<()> {
        Ok(())
    }

    /// Deposit usdc tokens into user's margin account
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        Ok(())
    }

    /// Withdrawal usdc tokens from user's margin account
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        Ok(())
    }

    /// Re-calculate user's margin requirement
    pub fn user_margin_calculate(ctx: Context<MarginContext>) -> Result<()> {
        Ok(())
    }

    /// Fund settlement - cranker function
    /// Record pnl for one user on one optifi market(one instruments)
    pub fn record_pnl_for_one_user(ctx: Context<RecordPnLForOneUser>) -> Result<()> {
        Ok(())
    }

    /// Fund settlement - cranker function
    /// Settle fund for one user for all markets with same expiry date - simultaneous settlement
    pub fn settle_fund_for_one_user(ctx: Context<SettleMarketFundForOneUser>) -> Result<()> {
        Ok(())
    }

    /// Fund settlement - cranker function
    /// Record pnl for amm on one optifi market(one instruments)
    pub fn record_pnl_for_amm(ctx: Context<RecordPnLForAmm>, amm_authority_bump: u8) -> Result<()> {
        Ok(())
    }

    /// Fund settlement - cranker function
    /// Settle fund for one user for all markets with same expiry date - simultaneous settlement
    pub fn settle_fund_for_amm(ctx: Context<SettleMarketFundForAmm>) -> Result<()> {
        Ok(())
    }

    /// Clean the expired instruments for user
    pub fn clean_expired_instruments_for_user(ctx: Context<CleanInstrumentForUser>) -> Result<()> {
        Ok(())
    }

    ////// AMM //////
    /// Initialize AMM
    pub fn initialize_amm(
        ctx: Context<InitializeAMM>,
        bump: u8,
        data: InitializeAMMData,
    ) -> Result<()> {
        Ok(())
    }

    /// Deposit funds to AMM
    pub fn amm_deposit(ctx: Context<DepositToAMM>, amount: u64) -> Result<()> {
        Ok(())
    }

    /// Add a request to withdraw queue
    pub fn add_withdraw_request(ctx: Context<AddAmmWithdrawReqeust>, amount: u64) -> Result<()> {
        Ok(())
    }

    /// Consume withdraw queue
    pub fn consume_withdraw_queue(ctx: Context<ConsumeAmmWithdrawReqeust>) -> Result<()> {
        Ok(())
    }

    /// Sync AMM opsitions
    pub fn amm_sync_positions(ctx: Context<SyncPositions>, instrument_index: u16) -> Result<()> {
        Ok(())
    }

    /// Sync AMM Future opsitions
    pub fn amm_sync_future_positions<'info>(
        ctx: Context<'_, '_, '_, 'info, SyncFuturePositions<'info>>,
        perp_market_index: u8,
    ) -> Result<()> {
        Ok(())
    }

    /// Place AMM future orders
    pub fn amm_update_future_orders<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateFutureOrder<'info>>,
    ) -> Result<()> {
        Ok(())
    }

    /// Calculate AMM delta
    pub fn amm_calculate_delta(ctx: Context<CalculateAmmDelta>) -> Result<()> {
        Ok(())
    }

    /// Calculate orders to update and save the orders in proposal
    pub fn amm_calculate_proposal(ctx: Context<CalculateAmmProposal>) -> Result<()> {
        Ok(())
    }

    /// Update AMM orders (place new orders)
    pub fn amm_update_orders(
        ctx: Context<UpdateAmmOrdersV3>,
        order_limit: u16,
        instrument_index: u16,
        amm_authority_bump: u8,
        market_auth_bump: u8,
    ) -> Result<()> {
        Ok(())
    }

    /// Remove instrument for AMM
    pub fn amm_remove_instrument(ctx: Context<RemoveOptiFiMarketForAMM>) -> Result<()> {
        Ok(())
    }
    /// Add instrument for AMM
    pub fn amm_add_instrument(ctx: Context<AddOptiFiMarketForAMM>) -> Result<()> {
        Ok(())
    }

    ////// Margin Stress //////
    pub fn margin_stress_init(
        ctx: Context<InitMarginStressContext>,
        bump: u8,
        asset: u8,
    ) -> Result<()> {
        Ok(())
    }

    pub fn margin_stress_calculate(ctx: Context<CalculateMarginStressContext>) -> Result<()> {
        Ok(())
    }

    ////// Liquidation //////
    pub fn init_liquidation(ctx: Context<InitializeLiquidation>) -> Result<()> {
        Ok(())
    }

    pub fn liquidation_register(ctx: Context<RegisterLiquidationMarket>) -> Result<()> {
        Ok(())
    }

    pub fn liquidation_place_order(ctx: Context<LiquidationPlaceOrder>) -> Result<()> {
        Ok(())
    }

    pub fn liquidation_settle_order(ctx: Context<LiquidationSettleOrder>) -> Result<()> {
        Ok(())
    }

    pub fn liquidation_to_amm(ctx: Context<LiquidationToAmm>) -> Result<()> {
        Ok(())
    }

    ////// Market Maker //////
    pub fn register_market_maker(ctx: Context<RegisterMarketMaker>, bump: u8) -> Result<()> {
        Ok(())
    }

    pub fn mm_post_only_order(
        ctx: Context<MmPostOnlyOrderContext>,
        side: OrderSide,
        limit: u64,
        max_coin_qty: u64,
        max_pc_qty: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn mm_calculation(ctx: Context<MmCalculationContext>) -> Result<()> {
        Ok(())
    }

    pub fn mm_cancel_order(ctx: Context<MMCancelOrderContext>) -> Result<()> {
        Ok(())
    }

    pub fn mm_settle_penalty_reward(ctx: Context<MmSettlePenaltyRewardContext>) -> Result<()> {
        Ok(())
    }

    pub fn schedule_market_maker_withdrawal(
        ctx: Context<ScheduleMarketMakerWithdrawal>,
        amount: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn execute_market_maker_withdrawal(
        ctx: Context<ExecuteMarketMakerWithdrawal>,
    ) -> Result<()> {
        Ok(())
    }

    ////// Authority //////
    pub fn withdraw_usdc_fee_pool(ctx: Context<WithdrawUsdcFeePool>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn update_exchange_authority(
        ctx: Context<UpdateExchangeAuthority>,
        new_authority: Pubkey,
    ) -> Result<()> {
        Ok(())
    }

    pub fn update_og_nft_mint(
        ctx: Context<UpdateOgNftMint>,
        og_nft_mint: Option<Pubkey>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn update_user_deposit_limit(
        ctx: Context<UpdateDepositLimit>,
        new_amount: Option<u64>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn update_oracle(
        ctx: Context<UpdateOracle>,
        asset: Asset,
        spot_oracle: Option<Pubkey>,
        iv_oracle: Option<Pubkey>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn set_delegation(ctx: Context<SetDelegation>, delegatee: Option<Pubkey>) -> Result<()> {
        Ok(())
    }
}
