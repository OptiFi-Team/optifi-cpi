use crate::prelude::*;

#[account]
#[derive(Default)]
pub struct Exchange {
    /// id of the OptiFi Exchange
    pub uuid: String,
    /// OptiFi Exchange version
    pub version: u32,
    /// the authority address
    pub exchange_authority: Pubkey,
    /// the recognized usdc token mint
    pub usdc_mint: Pubkey,
    /// usdc central pool for fund settlement
    pub usdc_central_pool: Pubkey,
    /// usdc fee pool
    pub usdc_fee_pool: Pubkey,
    /// user account id counter
    pub user_account_id_counter: u64,
    /// oracle data by assets
    pub oracle: Vec<OracleData>,
    /// a list of all created optifi markets, it should be updated when new market is created
    pub markets: Vec<OptifiMarketKeyData>,
    // a list of all created instruments groups, it should be updated when new group is created
    pub instrument_common: Vec<InstrumentCommon>,
    // a list of all created instruments, it should be updated when new instrument is created
    pub instrument_unique: Vec<Vec<InstrumentUnique>>,

    /// pubkey of og nft mint. None means og nft mode is turned off
    pub og_nft_mint: Option<Pubkey>,

    /// user deposit limit
    pub user_deposit_limit: Option<u64>,

    /// operation authority address
    pub operation_authority: Pubkey,

    /// iv authority address
    pub iv_authority: Pubkey,
}

impl Exchange {
    /// calc the space to be allocated
    pub fn get_space_allocation() -> usize {
        // 8 + 4
        //     + 32 * 4
        //     + 8
        //     + (1 + 33 + 33) * (asset + 1)
        //     + (32 + 8 + 1) * strike * asset * 2
        //     + (1 + 8 + 1) * asset * 2
        //     + (4 + 32 * 2) * strike * asset * 2
        10240
    }

    pub fn get_instrument_data(
        &self,
        instrument_pubkey: &Pubkey,
    ) -> Option<(InstrumentCommon, u32, bool)> {
        for (index, uniques) in self.instrument_unique.iter().enumerate() {
            for unique in uniques {
                for (is_call, k) in unique.instrument_pubkeys.iter().enumerate() {
                    if k == instrument_pubkey {
                        return Some((self.instrument_common[index], unique.strike, is_call != 0));
                    }
                }
            }
        }

        return None;
    }

    pub fn get_markets(&self, asset: Asset) -> (Vec<Pubkey>, Vec<u64>, Vec<u8>, Vec<u64>) {
        let mut instrument_pubkey: Vec<Pubkey> = vec![];
        let mut strikes: Vec<u64> = vec![];
        let mut is_call: Vec<u8> = vec![];
        let mut expiry_date: Vec<u64> = vec![];

        for (index, ic) in self.instrument_common.iter().enumerate() {
            if asset != ic.asset {
                continue;
            }
            for uniques in self.instrument_unique[index].iter() {
                instrument_pubkey =
                    [instrument_pubkey, uniques.instrument_pubkeys.to_vec()].concat();

                strikes = [strikes, [uniques.strike as u64; 2].to_vec()].concat();

                is_call = [is_call, [0, 1].to_vec()].concat();

                expiry_date = [expiry_date, [ic.expiry_date; 2].to_vec()].concat();
            }
        }

        (instrument_pubkey, strikes, is_call, expiry_date)
    }

    pub fn get_oracle(&self, asset: Asset) -> &OracleData {
        self.oracle.iter().find(|o| o.asset == asset).unwrap()
    }
}

/// only keep the key data for a created Instrument
#[derive(Debug, Copy, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct OracleData {
    pub asset: Asset,
    /// trusted oracle account for sopt price
    pub spot_oracle: Option<Pubkey>,
    /// trusted oracle account for iv
    pub iv_oracle: Option<Pubkey>,
}

/// keep the common data for an instrument group
#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
pub struct InstrumentCommon {
    /// underlying asset
    pub asset: Asset, // 1 bytes
    /// expiry date of the instrument, unix timestamp
    pub expiry_date: u64, // 8 bytes

    pub expiry_type: ExpiryType, // 1 byte
}

/// keep the unique data for an instrument
#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
pub struct InstrumentUnique {
    /// strike price of the instrument
    pub strike: u32, // 4 bytes
    /// instrument pubkey (0: put 1: call)
    pub instrument_pubkeys: [Pubkey; 2],
}

/// only keep the key data for a created OptiFi Market
#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub struct OptifiMarketKeyData {
    /// pubkey of created optifi market
    pub optifi_market_pubkey: Pubkey,
    /// expiry date of the instrument which is listed on this market
    pub expiry_date: u64,
    /// whether the optitfi market is stopped, which may be updated when the listing instruments is expired
    pub is_stopped: bool,
}
