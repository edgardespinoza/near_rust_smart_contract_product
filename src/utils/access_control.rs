use std::collections::HashMap;

use near_sdk::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
// see: https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/access/AccessControl.sol
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccessControl {

    pub roles : HashMap<String, Vec<String>>
    
}


impl AccessControl {
    
    pub fn has_role(&mut self, role: &str,  account:&AccountId) -> bool {
        let result = self.roles.get(role);
        match result {
            Some(x) => {
                x.contains(&account)
            },
           
            None   => false ,
        }
    }


    pub fn  setup_role(&mut self,  role: String,  account:AccountId)  {
        if !self.has_role( &role, &account) {
          
            let result = self.roles.get(&role);
            match result {
                Some( x) => {
                    let mut item= x.clone();
                    item.push(account);
                    self.roles.insert(role, item);

                },
               
                None =>    {
                    let v:Vec<String> = vec![account.to_string()];
                    self.roles.insert(role, v);
                }
                ,
            }
        }
    }
    
}