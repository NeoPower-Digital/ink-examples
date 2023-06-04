/// Withdraws the transferred value from the balance of the `caller`
#[ink(message)]
pub fn withdraw(&mut self, withdrawal_amount: Option<Balance>) -> Result<(), ContractError> {
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

    // self.env().emit_event(Withdrawn {
    //     to: caller,
    //     balance: withdrawal_amount,
    // });

    Ok(())
}
