
use std::collections::HashMap;

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{env, near_bindgen, setup_alloc};

setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Product {
    records: HashMap<String, Item>,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Item {
     price: u128,
     stock: u8,
     enabled: bool
}

#[near_bindgen]
impl Product{
    pub fn set_products(&mut self, address:String, price: u128, stock:u8){
        let owner = env::signer_account_id();

        // Use env::log to record logs permanently to the blockchain!
        env::log(format!("set_product '{}' for account '{}'", address, owner,).as_bytes());

        let item = Item {price, stock, enabled:true};

        self.records.insert(address, item);

    }

    pub fn get_products(&mut self, address:String) -> Option<&Item>{
         self.records.get(&address)
    }

    pub fn delete_products(&mut self, address:String) {
        self.records.remove(&address);
    }
}


/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn set_then_get_product() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Product::default();
        
        contract.set_products("0x1".to_string(), 12345, 12);
       
       let result = contract.get_products("0x1".to_string());
       
       let val = match result {
            // The division was valid
            Some(x) => {
                println!("Result: price:{}, stock:{}", x.price, x.stock);
                x.price
            },
            // The division was invalid
            None    => {
                println!("Not Exists");
                0
            },
          
        };

        assert_eq!(12345, val );
       
    }

    #[test]
    fn get_default_product() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let mut contract = Product::default();
        
        let result = contract.get_products("0x2".to_string());
       
        let val = match result {
             // The division was valid
             Some(x) => {
                 println!("Result: price:{}, stock:{}", x.price, x.stock);
                 x.price
             },
             // The division was invalid
             None    => {
                 println!("Not Exists");
                 0
             },
           
         };

        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
           0,
           val
        );
    }

    #[test]
    fn set_delete_product() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Product::default();
        
        contract.set_products("0x1".to_string(), 12345, 12);
       
        contract.delete_products("0x1".to_string());
       
       let result = contract.get_products("0x1".to_string());
       
       let val = match result {
            // The division was valid
            Some(x) => {
                println!("Result: price:{}, stock:{}", x.price, x.stock);
                x.price
            },
            // The division was invalid
            None    => {
                println!("Not Exists");
                0
            },
          
        };

        assert_eq!(0, val );
       
    }

    #[test]
    fn update_get_product() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Product::default();
        
        contract.set_products("0x1".to_string(), 12345, 12);
       
        contract.set_products("0x1".to_string(), 12345, 11);
       
       let result = contract.get_products("0x1".to_string());
       
       let val = match result {
            // The division was valid
            Some(x) => {
                println!("Result: price:{}, stock:{}", x.price, x.stock);
                x.stock
            },
            // The division was invalid
            None    => {
                println!("Not Exists");
                0
            },
          
        };

        assert_eq!(11, val );
       
    }
}
