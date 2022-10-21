pub mod models;
pub mod sales_view;
mod tests;

use near_sdk::{near_bindgen, env, AccountId,BorshStorageKey };
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::collections::UnorderedSet;
use models::{TicketSale, CollectionId};


#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    
    SalesStorageKey,
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    
    ticket_sales: UnorderedSet<TicketSale>,  
}


// Define the default, which automatically initializes the contract
impl Default for Contract{

    fn default() -> Self{
        Self{ ticket_sales :  UnorderedSet::new(StorageKey::SalesStorageKey)}
    }
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn init() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        
        Self{ ticket_sales :  UnorderedSet::new(StorageKey::SalesStorageKey)}
    }

}


#[near_bindgen]
impl Contract {

    pub fn insert_ticket_sale(&mut self, collection_id : CollectionId, token_id : TokenId, mint_by : AccountId){

        self.ticket_sales.insert(&TicketSale{
            collection_id : collection_id,
            token_id : token_id,
            mint_by : mint_by,
            date : Some(env::block_timestamp()),

        });
    }
}





