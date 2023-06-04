/// Deposits the transferred value as the balance of the `caller`
#[ink(message, payable)]
pub fn deposit(&mut self) -> Result<(), ContractError> {
    let caller = self.get_caller();
    let transferred_funds: Balance = self.check_and_get_transferred_funds()?;
    let account_balance: Balance = self.get_balance_by_account().unwrap_or(0);

    let new_balance = account_balance + transferred_funds;

    self.balances.insert(caller, &new_balance);

    // self.env().emit_event(Deposited {
    //     from: caller,
    //     balance: transferred_funds,
    // });

    Ok(())
}
