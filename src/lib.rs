pub mod models;
pub mod mints_view;
pub mod mints_manage;
mod tests;

use near_sdk::{near_bindgen, env, AccountId,BorshStorageKey };
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::collections::UnorderedMap;
use models::{TicketMint, TicketMintId, CollectionId,TicketAttribute, TicketAttributeType};


#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    
    SalesStorageKey,
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    
    ticket_mints: UnorderedMap<TicketMintId,TicketMint>,  

    allowed_callers : Option<Vec<AccountId>>,
}


// Define the default, which automatically initializes the contract
impl Default for Contract{

    fn default() -> Self{
        Self{ ticket_mints :  UnorderedMap::new(StorageKey::SalesStorageKey)
        , allowed_callers: None}
    }
}

#[near_bindgen]
impl Contract {

    #[init]
    #[private]
    #[allow(dead_code)]
    pub (crate) fn test_init() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        
        Self{ ticket_mints :  UnorderedMap::new(StorageKey::SalesStorageKey),
        allowed_callers : Some(vec!["bob".parse().unwrap(), "alice".parse().unwrap()])}
    }

}


#[near_bindgen]
impl Contract {

    #[init]
    #[private]
    pub fn init(allowed_callers : Vec<AccountId>) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        
        let s = Self{ ticket_mints :  UnorderedMap::new(StorageKey::SalesStorageKey),
        allowed_callers : Some(allowed_callers.clone())};

        env::log_str(format!("Initialized with allowed callers {:?}", allowed_callers).as_str());
        return s; 

    }

}


#[near_bindgen]
impl Contract {

    fn panic_if_its_not_allowed_caller(&self) {

        let uw_allowed_callers = self.allowed_callers.clone().unwrap();

        if !uw_allowed_callers.contains(&env::predecessor_account_id()) {
            env::panic_str(format!("@{} Error : Caller {} is NOT allowed",
            env::current_account_id(),
            env::predecessor_account_id()).as_str());
        }
    }
}






