pub mod models;
pub mod mints_view;
pub mod mints_manage;
mod tests;

use near_sdk::{near_bindgen, env, AccountId,BorshStorageKey };
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::collections::UnorderedMap;
use models::{TicketMint, TicketMintId, CollectionId,TicketAttribute};


#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    
    SalesStorageKey,
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    
    ticket_mints: UnorderedMap<TicketMintId,TicketMint>,  
}


// Define the default, which automatically initializes the contract
impl Default for Contract{

    fn default() -> Self{
        Self{ ticket_mints :  UnorderedMap::new(StorageKey::SalesStorageKey)}
    }
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn init() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        
        Self{ ticket_mints :  UnorderedMap::new(StorageKey::SalesStorageKey)}
    }

}






