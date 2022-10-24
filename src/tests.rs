#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {

    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};
    use crate::*;
   
    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    // cargo test test_insert_mints -- --show-output
    #[test]
    fn test_insert_mints() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());

        let mut _contract = Contract::test_init();
        let acc_id0 = accounts(0);

        _contract.insert_ticket_mint(CollectionId {
            owner : acc_id0.clone(),
            title : "TC 01 Collection".to_string(),
            symbol : "TC1".to_string(),
        }, "088811".to_string(), accounts(1), None);


        _contract.insert_ticket_mint(CollectionId {
            owner : acc_id0.clone(),
            title : "TC 01 Collection".to_string(),
            symbol : "TC1".to_string(),
        }, "088812".to_string(), accounts(1), None);


        _contract.insert_ticket_mint(CollectionId {
            owner : acc_id0.clone(),
            title : "TC 01 Collection".to_string(),
            symbol : "TC1".to_string(),
        }, "088811".to_string(), accounts(1), None );


        _contract.set_ticket_mint_is_used(CollectionId {
            owner : acc_id0.clone(),
            title : "TC 01 Collection".to_string(),
            symbol : "TC1".to_string(),
        }, "088812".to_string(), accounts(1));


        let sales = _contract.get_ticket_mints_by(acc_id0, None, None);

        for sale in sales {

            print!("\nsales:{:?}", sale);
        }

        testing_env!(context.is_view(true).build());

    }


     
}