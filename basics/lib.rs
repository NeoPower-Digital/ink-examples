#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]
#[ink::contract]
mod basics {
    #[derive(PartialEq, Debug, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NoValueTransferred,
    }

    #[ink(storage)]
    pub struct Basics {
        balance: Balance,
    }

    impl Basics {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { balance: 0 }
        }

        #[ink(message)]
        pub fn get_balance(&self) -> Balance {
            self.balance
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) -> Result<(), Error> {
            let transferred_value: Balance = self.env().transferred_value();

            if transferred_value == 0 {
                return Err(Error::NoValueTransferred);
            }

            self.balance += transferred_value;

            Ok(())
        }
    }
}
