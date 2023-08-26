#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]
#[ink::contract]
pub mod chain_extension_contract {
    use random_crate::*;

    #[derive(scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ContractError {
        FailGetRandom,
    }

    #[ink(storage)]
    pub struct ChainExtensionContract {}

    impl ChainExtensionContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn get_random(&self, input: [u8; 32]) -> Result<[u8; 32], ContractError> {
            RandomCrate::fetch_random(input).map_err(|_| ContractError::FailGetRandom)
        }
    }
}
