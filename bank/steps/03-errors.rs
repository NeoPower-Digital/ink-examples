#[derive(PartialEq, Debug, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ContractError {
    YouAreNotAClient,
    AccountWithoutBalance,
    InsufficientFunds,
    ExpectedWithdrawalAmountExceedsAccountBalance,
    WithdrawTransferFailed,
}
