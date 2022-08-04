use anchor_lang::error_code;

#[error_code]
pub enum OptifiErrorCode {
    #[msg("the user account cannot be initialized")]
    AccountCannotInit,

    #[msg("the user account is invalid")]
    InvalidAccount,

    #[msg("the margin account is invalid")]
    InvalidMarginAccount,

    #[msg("the account is not owned by the payer")]
    UnauthorizedAccount,

    #[msg("the account balance is insufficient")]
    InsufficientFund,

    #[msg("Token transfer failed")]
    TokenTransferFailed,

    #[msg("the token vault is not owned by the payer")]
    UnauthorizedTokenVault,

    #[msg("the provided pda is invalid")]
    InvalidPDA,

    #[msg("Uuid must be exactly of 6 length")]
    UuidMustBeExactly6Length,

    #[msg("Numerical overflow error!")]
    NumericalOverflowError,

    #[msg("Insufficient Margin!")]
    InsufficientMargin,

    #[msg("Incorrect coin mint!")]
    IncorrectCoinMint,

    #[msg("Cannot record pnl befor markets is expired!")]
    CannotRecordPnLBeforeMarketsExpired,

    #[msg("Cannot settle fund befor markets has been stopped!")]
    CannotSettleFundBeforeMarketsStopped,

    #[msg("Cannot record pnl when market is stopped!")]
    CannotRecordPnLForStoppedMarket,

    #[msg("Cannot record pnl for delisted instrument!")]
    InstrumetIsDelisted,

    #[msg("Incorrect oracle account")]
    IncorrectOracleAccount,

    #[msg("the working state is wrong")]
    WrongState,

    #[msg("the instrument has already been done")]
    WrongInstrument,

    #[msg("no enough orders in proposal to execute")]
    NoEnoughOrdersInProposal,

    #[msg("cannot remove the instrument for amm")]
    CannotRemoveInstrumentForAMM,

    #[msg("cannot add the instrument for amm due to duplication")]
    DuplicateInstrumentForAMM,

    #[msg("User is not in liquidation")]
    UserNotInLiquidation,

    #[msg("User is not in cancel order status")]
    UserNotInCancelOrder,

    #[msg("User was already in liquidation")]
    UserAlreadyInLiquidation,

    #[msg("Instrument was already registered for liquidation")]
    InstrumentAlreadyRegisteredForLiquidation,

    #[msg("Users cannot place manual orders while their accounts are in liquidation")]
    CannotPlaceOrdersInLiquidation,

    #[msg("Users cannot cancel manual orders while their accounts are in liquidation")]
    CannotCancelOrdersInLiquidation,

    #[msg("Provided USDC pool is not central pool")]
    PoolNotCentralUSDCPool,

    #[msg("Invalid open orders market authority")]
    InvalidSerumAuthority,

    #[msg("Only one withdraw request allowed at one time")]
    WithdrawRequestInvalid,

    #[msg("User is already registered as market maker")]
    UserIsMarketMaker,

    #[msg("Market maker withdraw outside of valid window")]
    MMWithdrawNotInWindow,

    #[msg("Wrong asset")]
    WrongAsset,

    #[msg("Should update the margin stress again")]
    TimeOut,

    #[msg("The order is failed")]
    OrderFailed,

    #[msg("Wrong USDC fee account")]
    WrongFeeAccount,
    #[msg("the mango account is invalid")]
    InvalidMangoAccount,

    #[msg("the mango token index is invalid")]
    InvalidMangoToken,

    #[msg("the length of open orders accounts is incorrect")]
    IncorrectOpenOrdersAccountsLength,

    #[msg("wrong perp market index")]
    WrongPerpMarketIndex,

    #[msg("asset of amm and the instrument are mismatched")]
    MismatchedAsset,

    #[msg("the instrument is expired")]
    InstrumentExpired,

    #[msg("duration of amm and the instrument are mismatched")]
    MismatchedDuration,

    #[msg("spot price is in strike interval")]
    DontNeedGenerateNextInstrument,

    #[msg("insufficient withdrawable lp token amount")]
    InsufficientWithdrawableLPAmount,

    #[msg("withdraw request queue is full")]
    WithdrawRequestQueueIsFull,

    #[msg("no withdraw request to process")]
    WithdrawRequestQueueIsEmpty,

    #[msg("mismatched withdraw request user id")]
    MismatchedWithdrawUserId,

    #[msg("amm withdraw not in correct time window")]
    AmmWithdrawNotInWindow,

    #[msg("withdrawal would breach amm delta")]
    WithdrawWouldBreachAmmDelta,
    #[msg("No OG NFT found")]
    NoOGNftFound,

    #[msg("Invalid OG NFT vault")]
    InvalidOGNftVault,

    #[msg("Invalid OG NFT mint")]
    InvalidOGNftMint,

    #[msg("unauthorized operation")]
    UnauthorizedOperation,

    #[msg("amm trading capacity is not enough")]
    InsufficientAmmCapacity,

    #[msg("cannot deposit more than the deposit limit")]
    CannotDepositMoreThanLimit,

    #[msg("Instrument index is wrong")]
    MismatchedInstrument,
}
