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
