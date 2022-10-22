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

impl TicketMint {

    pub fn update_attribute(&mut self, new_attribute : TicketAttribute,
    insert_if_not_exists : bool) {

        if self.attributes.is_some () {
     
            let mut uw_attrbs = self.attributes.clone().unwrap();

            let index = uw_attrbs.iter().position(|a| *a == new_attribute);

            if index.is_some() {

                uw_attrbs[index.unwrap()] = new_attribute;
                self.attributes = Some(uw_attrbs);
            }
            else {

                if insert_if_not_exists {

                    uw_attrbs.push(new_attribute);
                    self.attributes = Some(uw_attrbs);          
                }
            }
        }
       
    }
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

#[derive(BorshDeserialize, BorshSerialize,Debug, PartialEq,Serialize, Deserialize,Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum TicketAttributeType {

    IsUsed,

    DateUsed, 

    Price,

}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TicketAttribute {

    pub name : TicketAttributeType,

    pub value : Option<String>,
}

impl PartialEq for TicketAttribute {

    fn eq(&self, other: &Self) -> bool {
        self.name == other.name  
    }
}

