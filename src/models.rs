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


#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug)]
pub struct TicketMintId {

    pub collection_id : CollectionId,

    pub token_id : TokenId,

}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq, Clone)]
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