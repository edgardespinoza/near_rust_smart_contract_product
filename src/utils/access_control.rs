use near_sdk::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
// see: https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/access/AccessControl.sol
#[derive( BorshDeserialize, BorshSerialize)]
pub struct AccessControl {

    pub roles : LookupMap<String, Vec<String>>,
    
}


impl AccessControl {
    
    pub fn has_role(&mut self, role: &str,  account:&AccountId) -> bool {
        let result = self.roles.get(&role.to_string());
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
                    self.roles.insert(&role, &item);

                },
               
                None =>    {
                    let v:Vec<String> = vec![account.to_string()];
                    self.roles.insert(&role, &v);
                }
                ,
            }
        }
    }
    
}