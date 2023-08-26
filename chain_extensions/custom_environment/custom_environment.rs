use ink::env::{DefaultEnvironment, Environment};

use crate::error::RandomErr;

#[ink::chain_extension]
pub trait RandomExtension {
    type ErrorCode = RandomErr;

    #[ink(extension = 1101)]
    fn fetch_random(input: [u8; 32]) -> [u8; 32];
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = RandomExtension;
}
