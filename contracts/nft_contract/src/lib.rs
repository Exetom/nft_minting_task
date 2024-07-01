#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod nft_contract {
    use ink_storage::collections::HashMap as StorageHashMap;

    #[ink(storage)]
    pub struct NftContract {
        pub owner: AccountId,
        pub tokens: StorageHashMap<u64, AccountId>,
        pub total_supply: u64,
    }

    #[ink(event)]
    pub struct Minted {
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        token_id: u64,
    }

    impl NftContract {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            Self {
                owner,
                tokens: StorageHashMap::new(),
                total_supply: 0,
            }
        }

        #[ink(message)]
        pub fn mint(&mut self, to: AccountId) -> Result<u64, String> {
            let token_id = self.total_supply + 1;
            self.tokens.insert(token_id, to);
            self.total_supply = token_id;

            self.env().emit_event(Minted { to, token_id });

            Ok(token_id)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, token_id: u64) -> Result<(), String> {
            // Check ownership and perform transfer
            Ok(())
        }
    }
}