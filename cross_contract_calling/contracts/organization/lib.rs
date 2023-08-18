#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]
#[ink::contract]
mod organization {
    use ink::prelude::vec::Vec;
    use ink::{
        env::{
            call::{build_call, build_create, ExecutionInput, Selector},
            DefaultEnvironment,
        },
        selector_bytes,
    };
    use voting::VotingRef;

    #[ink(storage)]
    pub struct Organization {
        voting_contract: VotingRef,
    }

    impl Organization {
        #[ink(constructor)]
        pub fn new_with_ref(voting_contract_code_hash: Hash) -> Self {
            Self {
                voting_contract: VotingRef::new()
                    .code_hash(voting_contract_code_hash)
                    .endowment(0)
                    .salt_bytes(Vec::new()) // Sequence of bytes
                    .instantiate(),
            }
        }

        #[ink(constructor)]
        pub fn new_with_builder(voting_contract_code_hash: Hash) -> Self {
            Self {
                voting_contract: build_create::<VotingRef>()
                    .code_hash(voting_contract_code_hash)
                    .gas_limit(0)
                    .endowment(0)
                    .exec_input(ExecutionInput::new(Selector::new(selector_bytes!("new"))))
                    .salt_bytes(Vec::new()) // Sequence of bytes
                    .returns::<VotingRef>()
                    .instantiate(),
            }
        }

        #[ink(message)]
        pub fn vote_with_ref(&mut self, candidate: AccountId) {
            self.voting_contract.vote(candidate);
        }

        #[ink(message)]
        pub fn vote_with_builder(
            &mut self,
            candidate: AccountId,
            voting_contract_address: AccountId,
        ) {
            build_call::<DefaultEnvironment>()
                .call(voting_contract_address)
                .gas_limit(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(selector_bytes!("vote")))
                        .push_arg::<&[u8; 32]>(candidate.as_ref()),
                )
                .returns::<()>()
                .invoke();
        }

        #[ink(message)]
        pub fn get_votes_number_with_ref(&self, candidate: AccountId) -> u32 {
            self.voting_contract.get_votes_number(candidate)
        }

        #[ink(message)]
        pub fn get_votes_number_with_builder(
            &self,
            candidate: AccountId,
            voting_contract_address: AccountId,
        ) -> u32 {
            build_call::<DefaultEnvironment>()
                .call(voting_contract_address)
                .gas_limit(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(selector_bytes!("get_votes_number")))
                        .push_arg::<&[u8; 32]>(candidate.as_ref()),
                )
                .returns::<u32>()
                .invoke()
        }
    }
}
