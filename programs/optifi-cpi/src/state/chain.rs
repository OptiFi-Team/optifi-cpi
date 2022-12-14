use crate::prelude::*;

#[account]
pub struct Chain {
    /// underlying asset
    pub asset: u8, // 1 bytes
    /// option or future
    pub instrument_type: InstrumentType, // 1 bytes
    /// strike price of the instrument
    pub strike: u64, // 8 bytes
    /// expiry date of the instrument, unix timestamp
    pub expiry_date: u64, // 8 bytes
    /// Duration type
    pub duration: Duration, // 1 bytes
    /// Start date, as a unix timestamp
    pub start: u64, // 8 bytes
    /// Is this a perpetual contract? Only valid for futures
    pub expiry_type: ExpiryType, // 1 byte
    /// Is the instrument listed on market for trading
    pub is_listed_on_market: bool,
    /// contract size *10000 (f_to_u_repr)
    pub contract_size: u64,
}

impl Chain {
    /// calc the space to be allocated
    pub fn get_space_allocation() -> usize {
        // 56 when without strike and expiry_date
        // 72 add two more fields: strike and expiry_dat (u64)
        std::mem::size_of::<Self>() + 8
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Duration {
    Weekly,
    Monthly,
}

impl Default for Duration {
    fn default() -> Duration {
        Duration::Weekly
    }
}

impl TryFrom<u8> for Duration {
    type Error = ();

    fn try_from(v: u8) -> std::result::Result<Self, Self::Error> {
        match v {
            x if x == Duration::Weekly as u8 => Ok(Duration::Weekly),
            x if x == Duration::Monthly as u8 => Ok(Duration::Monthly),
            _ => Err(()),
        }
    }
}
