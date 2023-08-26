#![cfg_attr(not(feature = "std"), no_std)]

use error::RandomErr;

pub mod error;

pub struct RandomCrate;

impl RandomCrate {
    pub fn fetch_random(input: [u8; 32]) -> Result<[u8; 32], RandomErr> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(1101u32)
            .input::<[u8; 32]>()
            .output::<Result<[u8; 32], RandomErr>, true>()
            .handle_error_code::<RandomErr>()
            .call(&input)
    }
}
