#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod proxy {

    #[ink(storage)]
    pub struct Proxy {
        forward_to: AccountId,
        admin: AccountId,
    }

    impl Proxy {
        #[ink(constructor)]
        pub fn new(forward_to: AccountId) -> Self {
            Self {
                admin: Self::env().caller(),
                forward_to,
            }
        }

        #[ink(message)]
        pub fn change_forward_address(&mut self, new_address: AccountId) {
            assert_eq!(
                self.env().caller(),
                self.admin,
                "caller {:?} does not have sufficient permissions, only {:?} does",
                self.env().caller(),
                self.admin,
            );
            self.forward_to = new_address;
        }

        #[ink(message, payable, selector = _)]
        pub fn forward(&self) -> u32 {
            ink::env::call::build_call::<ink::env::DefaultEnvironment>()
                .call(self.forward_to)
                .transferred_value(self.env().transferred_value())
                .gas_limit(0)
                .call_flags(
                    ink::env::CallFlags::default()
                        .set_forward_input(true) // Redirigimos el input
                        .set_tail_call(true), // Retorna al usuario directamente el output
                )
                .try_invoke()
                .unwrap_or_else(|env_err| {
                    panic!(
                        "cross-contract call to {:?} failed due to {:?}",
                        self.forward_to, env_err
                    )
                })
                .unwrap_or_else(|lang_err| {
                    panic!(
                        "cross-contract call to {:?} failed due to {:?}",
                        self.forward_to, lang_err
                    )
                });
            unreachable!("the forwarded call will never return since `tail_call` was set");
        }
    }
}
