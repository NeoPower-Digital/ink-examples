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
