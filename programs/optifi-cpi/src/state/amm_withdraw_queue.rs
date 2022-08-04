use crate::prelude::*;

#[account(zero_copy)]
#[derive(Debug)]
pub struct AmmWithdrawRequestQueue {
    request_id_counter: u32,
    head: u32,
    tail: u32,
    requests: [WithdrawRequest; 5000],
}

impl AmmWithdrawRequestQueue {
    pub fn get_space_allocation() -> usize {
        std::mem::size_of::<Self>() + 8
    }
}
// #[derive(Default, Clone, Copy, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
#[zero_copy]
#[derive(Debug)]
pub struct WithdrawRequest {
    /// request id
    pub request_id: u32,
    /// id of user account
    pub user_account_id: u32,
    /// lp token amount to withdraw
    pub amount: u64,
    /// the timestamp when the request is received
    pub request_timestamp: u64,
}
