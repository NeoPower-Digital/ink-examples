#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RandomErr {
    FailGetRandom,
}

impl ink::env::chain_extension::FromStatusCode for RandomErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailGetRandom),
            _ => panic!("encountered unknown status code"),
        }
    }
}
