#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod meta_tx_contract {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
    };
    use ink_prelude::vec::Vec;

    #[ink(storage)]
    #[derive(Default)]
    pub struct MetaTxContract {
        owner: AccountId,
        nonces: StorageHashMap<AccountId, u64>,
    }

    #[ink(event)]
    pub struct ExecutedMetaTx {
        #[ink(topic)]
        executor: AccountId,
    }

    impl MetaTxContract {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                owner,
                nonces: Default::default(),
            }
        }

        #[ink(message)]
        pub fn execute_meta_tx(
            &mut self,
            signed_payload: Vec<u8>,
            signature: Vec<u8>,
            target: AccountId,
            input_data: Vec<u8>,
        ) -> Result<(), String> {
            // Verify signature (simplified for this example)
            let nonce = self.nonces.get(&target).unwrap_or(&0);
            self.nonces.insert(target, nonce + 1);

            // Emit the executed meta transaction event
            self.env().emit_event(ExecutedMetaTx {
                executor: self.env().caller(),
            });

            Ok(())
        }
    }
}

#[cfg(not(feature = "std"))]
#[no_mangle]
pub extern "C" fn deploy() {
    meta_tx_contract::MetaTxContract::default().deploy_owned();
}

#[cfg(test)]
mod tests {
    use super::*;
    use ink_env::test::ExecutionInput;

    #[test]
    fn test_execute_meta_tx() {
        let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
            .expect("Cannot get accounts");

        let mut contract = meta_tx_contract::MetaTxContract::new(accounts.alice);
        let meta_tx = MetaTx {
            signed_payload: vec![1, 2, 3],
            signature: vec![4, 5, 6],
            target: accounts.bob,
            input_data: vec![7, 8, 9],
        };

        let result = contract.execute_meta_tx(
            meta_tx.signed_payload.clone(),
            meta_tx.signature.clone(),
            meta_tx.target,
            meta_tx.input_data.clone(),
        );
        assert_eq!(result, Ok(()));
    }
}
