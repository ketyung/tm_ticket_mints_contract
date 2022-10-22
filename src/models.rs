use crate::*;


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TicketMint {

    pub collection_id : CollectionId,

    pub token_id : TokenId,

    pub attributes : Option<Vec<TicketAttribute>>,

    pub mint_by : Option<AccountId>,

    pub date : Option<u64>,
}

impl PartialEq for TicketMint {

    fn eq(&self, other: &Self) -> bool {
        self.collection_id == other.collection_id  
        && self.token_id == other.token_id 
    }
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct CollectionId {

    pub title : String, 

    pub symbol : String, 
    
    pub owner : AccountId, 
}



#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct TicketAttribute {

    pub name : String,

    pub value : Option<String>,
}