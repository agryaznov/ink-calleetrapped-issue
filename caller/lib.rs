// (c) 2021 Alexader Gryaznov
//
//! Demo of CalleeTrapped an issue with Ink!

#![cfg_attr(not(feature = "std"), no_std)]
use ink_lang as ink;

#[ink::contract]
mod caller {
    use ink_env::{
        call::{
            ExecutionInput, 
            Selector, 
            build_call as build_call,
            utils::ReturnType
        }
    };
    
    /// Defines the storage of the contract.
    #[ink(storage)]
    pub struct Caller {
        /// Callee contract address
        callee_contract_address: AccountId,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    /// Error types
    pub enum Error {
        SomeError,
    }
 

    impl Caller {
        /// Caller constructor.  
        #[ink(constructor)]
        pub fn new(
            callee_contract_address: AccountId,
        ) -> Self {
            Self { 
                callee_contract_address,
            }
        }

        /// Caller logic  
        /// Invoke flipper.flip()
        #[ink(message)]
        pub fn call_flipper(&self) -> Result<(), Error> {
            const SELECTOR: [u8; 4] = [0xC0, 0xDE, 0xCA, 0xFE];
            let selector = Selector::new(SELECTOR);
            let to = self.callee_contract_address; 
            let params = build_call::<Environment>()
                .callee(to)
                .gas_limit(50000)
                .exec_input(
                    ExecutionInput::new(selector)
                )
                .returns::<ReturnType<Result<(), Error>>>();

            match params.fire() {
                Ok(v) => {
                    ink_env::debug_println!(
                        "Received return value \"{:?}\" from contract {:02X?}",
                        v,
                        to
                    );
                    Ok(())
                }
                Err(e) => {
                    match e {
                        ink_env::Error::CodeNotFound
                        | ink_env::Error::NotCallable => {
                            // Our recipient wasn't a smart contract, so there's nothing more for
                            // us to do
                            ink_env::debug_println!(
                                "Recipient at {:02X?} from is not a smart contract ({:?})", 
                                to, 
                                e
                            );
                            Err(Error::SomeError)
                        }
                        _ => {
                            // We got some sort of error from the call to our recipient smart
                            // contract, and as such we must revert this call
                            let msg = ink_prelude::format!(
                                "Got error \"{:?}\" while trying to call {:?} with SELECTOR: {:?}",
                                e,
                                to,
                                selector.to_bytes()
                            
                            );
                            ink_env::debug_println!("{}", &msg);
                            panic!("{}", &msg)
                        }
                    }
                }
            }

        }
    }
}
