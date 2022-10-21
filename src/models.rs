use crate::*;


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TicketSale {

    pub collection_id : CollectionId,

    pub token_id : TokenId,

    pub mint_by : AccountId,

    pub date : Option<u64>,
}

impl PartialEq for TicketSale {

    fn eq(&self, other: &Self) -> bool {
        self.collection_id == other.collection_id  
        && self.token_id == other.token_id 
        && self.mint_by == other.mint_by 
    }
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct CollectionId {

    pub title : String, 

    pub symbol : String, 
    
    pub owner : AccountId, 

}