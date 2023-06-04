#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod bank {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Bank {
        balances: Mapping<AccountId, Balance>,
    }

    impl Bank {}
}
