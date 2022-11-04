use crate::*;
use itertools::Itertools;

#[near_bindgen]
impl Contract {

    pub fn get_ticket_mints_by(&self,
    owner : AccountId, offset : Option<usize>, limit : Option<usize>) -> Vec<TicketMint>
    {
        let u = self.ticket_mints.values_as_vector()
        .iter()
        .filter(|s| s.collection_id.owner == owner )
        .sorted_by(|a, b| Ord::cmp(&b.date, &a.date))
        .skip(offset.unwrap_or(0))
        .take(limit.unwrap_or(10))
        .collect::<Vec<TicketMint>>();
        
        return u; 
    }


    pub fn get_ticket_mints_of(&self,collection_id : CollectionId, 
    offset : Option<usize>, limit : Option<usize>) -> Vec<TicketMint>
    {

        let u = self.ticket_mints.values_as_vector()
        .iter()
        .filter(|s| s.collection_id == collection_id )
        .sorted_by(|a, b| Ord::cmp(&b.date, &a.date))
        .skip(offset.unwrap_or(0))
        .take(limit.unwrap_or(10))
        .collect::<Vec<TicketMint>>();

        
        return u; 
    }
}