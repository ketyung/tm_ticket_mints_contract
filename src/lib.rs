pub mod models;
pub mod sales_view;
mod tests;

use near_sdk::{near_bindgen, env, AccountId,BorshStorageKey };
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::collections::UnorderedSet;
use models::{TicketMint, CollectionId,TicketAttribute};


#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    
    SalesStorageKey,
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    
    ticket_mints: UnorderedSet<TicketMint>,  
}


// Define the default, which automatically initializes the contract
impl Default for Contract{

    fn default() -> Self{
        Self{ ticket_mints :  UnorderedSet::new(StorageKey::SalesStorageKey)}
    }
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn init() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        
        Self{ ticket_mints :  UnorderedSet::new(StorageKey::SalesStorageKey)}
    }

}


#[near_bindgen]
impl Contract {

    pub fn insert_ticket_mint(&mut self, collection_id : CollectionId, 
        token_id : TokenId, mint_by : AccountId){

        let attributes = vec![
        TicketAttribute {
            name : "is_used".to_string(),
            value : Some("false".to_string()),
        }, 
        TicketAttribute {

            name : "date_minted".to_string(),
            value : Some(format!("{}",env::block_timestamp()).to_string()),
        }];

        self.ticket_mints.insert(&TicketMint{
            collection_id : collection_id,
            token_id : token_id,
            attributes : Some(attributes), 
            mint_by : mint_by,
            date : Some(env::block_timestamp()),

        });
    }
}





