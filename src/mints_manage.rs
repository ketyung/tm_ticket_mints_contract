use crate::*;


#[near_bindgen]
impl Contract {

    pub fn insert_ticket_mint(&mut self, collection_id : CollectionId, 
        token_id : TokenId, mint_by : AccountId) -> bool{

        let id = TicketMintId {
            collection_id : collection_id.clone(),
            token_id : token_id.clone(),
        };

        if self.ticket_mints.get(&id).is_some() {

            return false;
        }
        

        let attributes = vec![
        TicketAttribute {
            name : "is_used".to_string(),
            value : Some("false".to_string()),
        }, 
        TicketAttribute {
            name : "date_minted".to_string(),
            value : Some(format!("{}",env::block_timestamp()).to_string()),
        }];


        self.ticket_mints.insert(
            &id,   
            &TicketMint{
            collection_id : collection_id,
            token_id : token_id,
            attributes : Some(attributes), 
            mint_by : Some(mint_by),
            date : Some(env::block_timestamp()),
        });

        return true;
    }
}



#[near_bindgen]
impl Contract {

    pub fn set_ticket_mint_is_used(&mut self, collection_id : CollectionId, 
        token_id : TokenId){

        let id = TicketMintId {
            collection_id : collection_id,
            token_id : token_id,
        };
    
        let ticket_mint = self.ticket_mints.get(&id);

        if ticket_mint.is_none() {

            env::panic_str(format!("No such ticket mint {:?}", ticket_mint).as_str());
        }


    }
}