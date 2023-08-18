#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod erc20;

#[ink::contract]
mod token {
    use crate::erc20::{Erc20, Error};
    use ink::storage::Mapping;

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[ink(storage)]
    pub struct Token {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    impl Token {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: caller,
                value: total_supply,
            });

            Self {
                total_supply,
                balances,
                allowances: Mapping::default(),
            }
        }

        fn get_balance(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or(0)
        }

        fn get_allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or(0)
        }

        fn transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<(), Error> {
            let from_balance = self.get_balance(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(from, &(from_balance - value));

            let to_balance = self.get_balance(to);
            self.balances.insert(to, &(to_balance + value));

            self.env().emit_event(Transfer {
                from: Some(from),
                to,
                value,
            });

            Ok(())
        }
    }

    impl Erc20 for Token {
        #[ink(message)]
        fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> Balance {
            self.get_balance(owner)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.get_allowance(owner, spender)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Error> {
            let from = Self::env().caller();
            self.transfer_from_to(from, to, value)
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), Error> {
            let owner = Self::env().caller();

            self.allowances.insert((owner, spender), &value);

            Self::env().emit_event(Approval {
                owner,
                spender,
                value,
            });

            Ok(())
        }

        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<(), Error> {
            let caller = Self::env().caller();

            let allowance = self.get_allowance(from, caller);

            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }

            self.transfer_from_to(from, to, value)?;

            self.allowances.insert((from, caller), &(allowance - value));

            Ok(())
        }
    }
}
