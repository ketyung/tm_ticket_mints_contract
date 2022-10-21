use crate::*;


#[near_bindgen]
impl Contract {

    pub fn get_ticket_sales_by(&self,
    owner : AccountId, offset : Option<usize>, limit : Option<usize>) -> Vec<TicketSale>
    {
        let u = self.ticket_sales.as_vector()
        .iter()
        .filter(|s| s.collection_id.owner == owner )
        .skip(offset.unwrap_or(0))
        .take(limit.unwrap_or(10))
        .collect::<Vec<TicketSale>>();
        
        return u; 
    }
}