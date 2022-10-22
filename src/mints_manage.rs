use crate::*;


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
