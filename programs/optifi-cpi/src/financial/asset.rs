use super::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Asset {
    Bitcoin,
    Ethereum,
    USDC,
    Solana,
}

impl Asset {
    pub fn get_decimal(self) -> u32 {
        match self {
            x if x == Asset::Bitcoin => 2,
            x if x == Asset::Ethereum => 1,
            x if x == Asset::Solana => 1,
            _ => 0,
        }
    }
}

impl Default for Asset {
    fn default() -> Asset {
        Asset::Bitcoin
    }
}

impl TryFrom<u8> for Asset {
    type Error = ();

    fn try_from(v: u8) -> std::result::Result<Asset, ()> {
        match v {
            x if x == Asset::Bitcoin as u8 => Ok(Asset::Bitcoin),
            x if x == Asset::Ethereum as u8 => Ok(Asset::Ethereum),
            x if x == Asset::USDC as u8 => Ok(Asset::USDC),
            x if x == Asset::Solana as u8 => Ok(Asset::Solana),
            _ => Err(()),
        }
    }
}
