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


    pub fn get_ticket_mints_count_for(&self, owner : AccountId,
    date_ranges : Vec<DateRange>) -> Vec<SalesCount> {

        let mut cnts : Vec<SalesCount> = Vec::new();

        for dr in date_ranges.iter() {

            let cnt = self.get_ticket_mints_count(owner.clone(), 
            dr.start_date_timestamp, dr.end_date_timestamp);
            let sc = SalesCount {
                date : dr.date.clone(),
                count : cnt, 
            };

            cnts.push(sc);
        }

        cnts
    
    }


    pub fn get_ticket_mints_count(&self, owner : AccountId,
    date_start :  Option<u64>, date_end :  Option<u64>) -> usize
    {
    
        self.ticket_mints.values_as_vector()
        .iter()
        .filter(|s| s.collection_id.owner == owner && 
        s.date >= date_start && s.date <= date_end )
        .count()
        
    }
    
    pub fn get_tickets_buyers(&self, owner : AccountId,
        offset : Option<usize>, limit : Option<usize>) -> Vec<AccountId>
    {
        let grouped_data  = 
        self.ticket_mints.values_as_vector()
        .iter()
        .filter(|s| s.collection_id.owner == owner )
        .sorted_by(|a, b| Ord::cmp(&b.date, &a.date))
        .unique_by(|x| x.clone().mint_by)
        .skip(offset.unwrap_or(0))
        .take(limit.unwrap_or(10))
        .collect::<Vec<TicketMint>>();


        let mut accs : Vec<AccountId>= Vec::new();

        for a in grouped_data.into_iter() {

            if !accs.contains(&a.mint_by.clone().unwrap()){
                accs.push(a.mint_by.unwrap());
            }
        
        }
        
        return accs;
        
    }


}