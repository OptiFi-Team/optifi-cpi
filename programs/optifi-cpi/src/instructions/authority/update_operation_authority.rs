use crate::prelude::*;

#[derive(Accounts)]
pub struct UpdateOperationAuthority<'info> {
    #[account(mut)]
    pub optifi_exchange: Box<Account<'info, Exchange>>,

    #[account(signer,
        constraint= optifi_exchange.exchange_authority == authority.key() || optifi_exchange.operation_authority == authority.key() @ OptifiErrorCode::UnauthorizedOperation
    )]
    pub authority: AccountInfo<'info>,
}
