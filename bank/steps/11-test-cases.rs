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
