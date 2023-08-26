#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod custom_environment;
pub mod error;

#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]
#[ink::contract(env = crate::custom_environment::CustomEnvironment)]
mod chain_extension_contract {
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
            self.env()
                .extension()
                .fetch_random(input)
                .map_err(|_| ContractError::FailGetRandom)
        }
    }
}
