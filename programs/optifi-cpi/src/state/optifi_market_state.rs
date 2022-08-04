use crate::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct OptifiMarket {
    /// id of the optifi market, we may have markets with id from 1 ~ 50
    pub optifi_market_id: u16,
    /// the serum orderbook market which is used to swap instrument spl token and quote token
    pub serum_market: Pubkey,
    /// the instrument which is listed on this market
    pub instrument: Pubkey,
    /// instrumnet long spl token which would be sent to instrument buyers
    pub instrument_long_spl_token: Pubkey,
    /// instrumnet short spl token which would be minted to instrument seller
    pub instrument_short_spl_token: Pubkey,
    /// whether the optitfi market is stopped, which may be updated when the listing instruments is expired
    pub is_stopped: bool,
    /// bump seed which is used to generate this optifi market address
    pub bump: u8,
}

impl OptifiMarket {
    /// calc the space to be allocated
    pub fn get_space_allocation() -> usize {
        std::mem::size_of::<Self>() + 8 + 200
    }
}
