#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod meta_tx_contract {
    use ink_storage::collections::HashMap as StorageHashMap;

    #[ink(storage)]
    pub struct MetaTxContract {
        pub owner: AccountId,
        pub nonces: StorageHashMap<AccountId, u64>,
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
                nonces: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn execute_meta_tx(
            &mut self,
            signed_payload: Vec<u8>,
            signature: Vec<u8>,
            target: AccountId,
            input_data: Vec<u8>
        ) -> Result<(), String> {
            // Verify signature (simplified for this example)
            let nonce = self.nonces.get(&target).unwrap_or(&0);
            self.nonces.insert(target, nonce + 1);

            // Execute the transaction
            self.env().emit_event(ExecutedMetaTx {
                executor: self.env().caller(),
            });

            Ok(())
        }
    }
}
