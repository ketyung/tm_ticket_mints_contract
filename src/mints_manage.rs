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
            mint_by : Some(mint_by),
            date : Some(env::block_timestamp()),

        });
    }
}



#[near_bindgen]
impl Contract {

    pub fn set_ticket_mint_is_used(&mut self, collection_id : CollectionId, 
        token_id : TokenId){

        let to_match_tm = TicketMint{
            collection_id : collection_id,
            token_id : token_id,
            attributes : None, 
            mint_by : None,
            date : None,
        };

        if !self.ticket_mints.contains(&to_match_tm) {

            env::panic_str(format!("No such ticket mint {:?}", to_match_tm).as_str());
        }


    }
}