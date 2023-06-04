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
