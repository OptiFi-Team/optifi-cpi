use crate::prelude::*;

#[derive(Accounts)]
pub struct WithdrawUsdcFeePool<'info> {
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    #[account(mut,
        constraint = usdc_fee_pool.key() == optifi_exchange.usdc_fee_pool @ OptifiErrorCode::WrongFeeAccount,
        constraint = accessor::authority(&usdc_fee_pool)?== get_central_usdc_pool_auth_pda(&optifi_exchange.key(), program_id).0
    )]
    pub usdc_fee_pool: AccountInfo<'info>,

    pub central_usdc_pool_auth: AccountInfo<'info>,

    #[account(mut,
        constraint= !withdraw_dest.data_is_empty() && withdraw_dest.lamports() > 0
    )]
    pub withdraw_dest: AccountInfo<'info>,

    #[account(signer,
        constraint= optifi_exchange.exchange_authority == authority.key()
    )]
    pub authority: AccountInfo<'info>,

    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
}
