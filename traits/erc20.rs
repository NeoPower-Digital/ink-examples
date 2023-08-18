use ink::{
    env::{DefaultEnvironment, Environment},
    primitives::AccountId,
};

pub type Balance = <DefaultEnvironment as Environment>::Balance;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    InsufficientBalance,
    InsufficientAllowance,
}

#[ink::trait_definition]
pub trait Erc20 {
    #[ink(message)]
    fn total_supply(&self) -> Balance;

    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> Balance;

    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance;

    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Error>;

    #[ink(message)]
    fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), Error>;

    #[ink(message)]
    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: Balance,
    ) -> Result<(), Error>;
}
