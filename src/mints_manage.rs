use crate::*;


#[near_bindgen]
impl Contract {

    pub fn insert_ticket_mint(&mut self, collection_id : CollectionId, 
    token_id : TokenId, mint_by : AccountId, price : Option<u128>,
    ticket_type : Option<String>) -> bool{

        self.panic_if_its_not_allowed_caller();

        let id = TicketMintId {
            collection_id : collection_id.clone(),
            token_id : token_id.clone(),
        };

        if self.ticket_mints.get(&id).is_some() {

            return false;
        }
        

        let attributes = vec![
        TicketAttribute {
            name : TicketAttributeType::IsUsed,
            value : Some("false".to_string()),
        }, 
        TicketAttribute {
            name : TicketAttributeType::Price,
            value : Some(format!("{}",price.unwrap_or(0)).to_string()),
        },
        TicketAttribute {
            name : TicketAttributeType::TicketType,
            value : ticket_type,
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
        token_id : TokenId, used_by : AccountId) -> bool{

        
        self.panic_if_its_not_allowed_caller();

        let id = TicketMintId {
            collection_id : collection_id,
            token_id : token_id,
        };
    
        let ticket_mint = self.ticket_mints.get(&id);

        if ticket_mint.is_none() {
            return false;
        }


        let mut uw_ticket_mint = ticket_mint.unwrap();
     
        uw_ticket_mint.update_attribute(TicketAttribute {
            name : TicketAttributeType::IsUsed,
            value : Some("true".to_string()),
        }, true );

        uw_ticket_mint.update_attribute(TicketAttribute {
            name : TicketAttributeType::UsedBy,
            value : Some(used_by.to_string()),
        }, true );

        uw_ticket_mint.update_attribute(TicketAttribute {
            name : TicketAttributeType::DateUsed,
            value : Some(format!("{}", env::block_timestamp())),
        }, true );

        self.ticket_mints.remove(&id);
        self.ticket_mints.insert(&id, &uw_ticket_mint);

        return true;
    }
}