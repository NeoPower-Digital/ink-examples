#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]
#[ink::contract]
mod bank {
    use ink::storage::Mapping;

    #[ink(event)]
    pub struct Deposited {
        from: AccountId,
        balance: Balance,
    }

    #[ink(event)]
    pub struct Withdrawn {
        to: AccountId,
        balance: Balance,
    }

    #[derive(PartialEq, Debug, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ContractError {
        YouAreNotAClient,
        AccountWithoutBalance,
        InsufficientFunds,
        ExpectedWithdrawalAmountExceedsAccountBalance,
        WithdrawTransferFailed,
    }

    #[ink(storage)]
    pub struct Bank {
        balances: Mapping<AccountId, Balance>,
    }

    impl Bank {
        /// Creates a new instance of our bank
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: Mapping::default(),
            }
        }

        /// Returns the balance of the `caller`
        #[ink(message)]
        pub fn get_balance_by_account(&self) -> Result<Balance, ContractError> {
            let caller = self.get_caller();

            match self.balances.get(caller) {
                Some(account_balance) => Ok(account_balance),
                None => Err(ContractError::YouAreNotAClient),
            }
        }

        /// Deposits the transferred value as the balance of the `caller`
        #[ink(message, payable)]
        pub fn deposit(&mut self) -> Result<(), ContractError> {
            let caller = self.get_caller();
            let transferred_funds: Balance = self.check_and_get_transferred_funds()?;
            let account_balance: Balance = self.get_balance_by_account().unwrap_or(0);

            let new_balance = account_balance + transferred_funds;

            self.balances.insert(caller, &new_balance);

            self.env().emit_event(Deposited {
                from: caller,
                balance: transferred_funds,
            });

            Ok(())
        }

        /// Withdraws the transferred value from the balance of the `caller`
        #[ink(message)]
        pub fn withdraw(
            &mut self,
            withdrawal_amount: Option<Balance>,
        ) -> Result<(), ContractError> {
            let caller = self.get_caller();
            let mut account_balance: Balance = self.get_balance_by_account()?;

            if account_balance == 0 {
                return Err(ContractError::AccountWithoutBalance);
            }

            let withdrawal_amount: Balance = withdrawal_amount.unwrap_or(account_balance);

            if withdrawal_amount > account_balance {
                return Err(ContractError::ExpectedWithdrawalAmountExceedsAccountBalance);
            }

            account_balance -= withdrawal_amount;
            self.balances.insert(caller, &account_balance);

            if self.env().transfer(caller, withdrawal_amount).is_err() {
                return Err(ContractError::WithdrawTransferFailed);
            }

            self.env().emit_event(Withdrawn {
                to: caller,
                balance: withdrawal_amount,
            });

            Ok(())
        }

        fn get_caller(&self) -> AccountId {
            self.env().caller()
        }

        fn check_and_get_transferred_funds(&self) -> Result<Balance, ContractError> {
            let transferred_funds: Balance = self.env().transferred_value();

            if transferred_funds == 0 {
                return Err(ContractError::InsufficientFunds);
            }

            Ok(transferred_funds)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn get_default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn init() -> (
            Bank,
            ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment>,
        ) {
            (Bank::new(), get_default_accounts())
        }

        fn set_caller(sender: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(sender);
        }

        #[ink::test]
        fn withdraw_works_for_valid_client() {
            // Arrange
            let (mut contract, accounts) = init();
            let caller = accounts.bob;
            let balance_amount = 1000;
            let withdrawal_amount = 600;

            contract.balances.insert(caller, &balance_amount);
            set_caller(caller);

            // Act
            contract.withdraw(Some(withdrawal_amount)).unwrap();
            let result = contract.balances.get(caller).unwrap();

            // Assert
            assert_eq!(result, balance_amount - withdrawal_amount);
        }

        #[ink::test]
        fn withdraw_fails_fails_for_invalid_client() {
            // Arrange
            let (mut contract, accounts) = init();
            let caller = accounts.bob;
            set_caller(caller);

            // Act
            let result = contract.withdraw(None);

            // Assert
            assert_eq!(result, Err(ContractError::YouAreNotAClient));
        }
    }
}
