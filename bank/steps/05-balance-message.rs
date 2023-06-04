/// Returns the balance of the `caller`
#[ink(message)]
pub fn get_balance_by_account(&self) -> Result<Balance, ContractError> {
    let caller = self.get_caller();

    match self.balances.get(caller) {
        Some(account_balance) => Ok(account_balance),
        None => Err(ContractError::YouAreNotAClient),
    }
}
