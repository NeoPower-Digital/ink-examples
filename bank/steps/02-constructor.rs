/// Creates a new instance of our bank
#[ink(constructor)]
pub fn new() -> Self {
    Self {
        balances: Mapping::default(),
    }
}
