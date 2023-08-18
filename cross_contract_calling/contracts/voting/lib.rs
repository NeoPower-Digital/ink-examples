#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::voting::VotingRef;

#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]
#[ink::contract]
mod voting {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Voting {
        votes: Mapping<AccountId, u32>,
    }

    impl Voting {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                votes: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn vote(&mut self, candidate: AccountId) {
            let candidate_votes = self.votes.get(candidate).unwrap_or(0);
            self.votes.insert(candidate, &(candidate_votes + 1));
        }

        #[ink(message)]
        pub fn get_votes_number(&self, candidate: AccountId) -> u32 {
            self.votes.get(candidate).unwrap_or(0)
        }
    }
}
