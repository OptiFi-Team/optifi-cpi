use super::*;
use std::result::Result;

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum InstrumentType {
    Put = 0,
    Call = 1,
}

impl TryFrom<u8> for InstrumentType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == InstrumentType::Put as u8 => Ok(InstrumentType::Put),
            x if x == InstrumentType::Call as u8 => Ok(InstrumentType::Call),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum ExpiryType {
    Standard = 0,
    Perpetual = 1,
}

impl TryFrom<u8> for ExpiryType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == ExpiryType::Standard as u8 => Ok(ExpiryType::Standard),
            x if x == ExpiryType::Perpetual as u8 => Ok(ExpiryType::Perpetual),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct OptionData {
    size: i32,
}

pub enum InstrumentData {}

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize)]
#[repr(u8)]
pub enum InstrumentExpiryType {
    Standard = 0,
    Perpetual = 1,
}

impl TryFrom<u8> for InstrumentExpiryType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == InstrumentExpiryType::Standard as u8 => Ok(InstrumentExpiryType::Standard),
            x if x == InstrumentExpiryType::Perpetual as u8 => Ok(InstrumentExpiryType::Perpetual),
            _ => Err(()),
        }
    }
}
