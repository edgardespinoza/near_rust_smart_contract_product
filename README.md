# Management Product
- Management Products is an smart contract where you can add, delete, update and get products with stock, using NEAR Protocol
- Add permision Ownable 
- Add AccessControl with roles

## Flow Business
[![](https://mermaid.ink/img/pako:eNp1UsFqwzAM_RXh08qaH8ihkG457NKV9bBLoGi21pk5dmYrhVL675OblmRlNdiI956eJdlHpYMhVapEPz15Tc8WdxHbxoMs1BwiVIAJKmc1wcPr-6p-mw1sh5Gtth16hnXWbFpB4Cl4jpIJBaxjML3mqdkyC5fhY4rVGav3NGBVsVg8rktIxNtuMEh_md1dxpAjphvyfA7h8q7DKjBB2JNUOBeBVAjBuwNo9LKdg5b4K5icOXEFdAyV1pTS0HdwAz4q8qqLYlbljryRlnXoPb-YkZYp2D3K_fWIjWA1gtc20ZitXEXbyYxGlaH_cuu7Y73JuRQhtZ4DNVctxRatkV9yzFCj-ItaalQpocH43ajGn0TXd0YMamPlUVX5iS7RXGHPYXPwWpUce7qKLt_sojr9Anbhx5g)](https://mermaid-js.github.io/mermaid-live-editor/edit/#pako:eNp1UsFqwzAM_RXh08qaH8ihkG457NKV9bBLoGi21pk5dmYrhVL675OblmRlNdiI956eJdlHpYMhVapEPz15Tc8WdxHbxoMs1BwiVIAJKmc1wcPr-6p-mw1sh5Gtth16hnXWbFpB4Cl4jpIJBaxjML3mqdkyC5fhY4rVGav3NGBVsVg8rktIxNtuMEh_md1dxpAjphvyfA7h8q7DKjBB2JNUOBeBVAjBuwNo9LKdg5b4K5icOXEFdAyV1pTS0HdwAz4q8qqLYlbljryRlnXoPb-YkZYp2D3K_fWIjWA1gtc20ZitXEXbyYxGlaH_cuu7Y73JuRQhtZ4DNVctxRatkV9yzFCj-ItaalQpocH43ajGn0TXd0YMamPlUVX5iS7RXGHPYXPwWpUce7qKLt_sojr9Anbhx5g)

----

## Prerequisites

-Make sure you have node.js installed 

```bash
   node.js installed => 12v
```

-Make sure you have a testnet account, if not, you can following the steps on next link to create one:

```
  https://docs.near.org/docs/develop/basics/create-account 
```



## Setting up
- install environment
  ```bash
   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   $ rustup target add wasm32-unknown-unknown
   $ npm install -g near-cli
   ```

# configuration the project
- ID is managment account in near,  change yyyy.testnet  for your account
-  ID_ALICER is who deployment the contract,
-  ID_BOB is who calling methods from contract,  
-  ID_EVE is who has permition,  
    ```bash
    $   export ID=yyyy.testnet
    $   export ID_ALICE=d1.$ID 
    $   export ID_BOB=d2.$ID 
    $   export ID_EVE=d3.$ID 
    ```
    
- to login and create new account
    ```bash
        $ near login 
        $ near create-account $ID_ALICE --masterAccount $ID --initialBalance 4
        $ near create-account $ID_BOB --masterAccount $ID --initialBalance 1
        $ near create-account $ID_EVE --masterAccount $ID --initialBalance 2
    ```

- Clean the folder target
  ```bash
   $ cargo clean 
  ```

- to build contract 
  ```bash
   $ cargo build --target wasm32-unknown-unknown --release
  ```
- to run test contract 
    ```bash
    $  cargo test -- --nocapture
    ```

- to deploy contract 
    ```bash
    $ near deploy  --initFunction 'new' --initArgs '{}'  --wasmFile target/wasm32-unknown-unknown/release/sales.wasm --accountId $ID_ALICE
    ``` 

- call smart contracts
  * Alice only can add product
    ```bash
      near call $ID_ALICE set_products '{"address":"0x1", "name":"zapato marca X", "price":12345,"stock":5}' --accountId $ID_ALICE
     ```

  * Everyone can get products
    ```bash
      near view $ID_ALICE get_products '{"address":"0x1"}' --accountId $ID_BOB
     ```

   * Alice set role "set product" to Eve
    ```bash
      near call $ID_ALICE add_role_set_product '{"account":"'$ID_EVE'"}' --accountId $ID_ALICE
     ```

     * Alice set role "del product" to Eve
    ```bash
      near call $ID_ALICE add_role_delete_product '{"account":"'$ID_EVE'"}' --accountId $ID_ALICE
    ```

  * Eve can now can add  or update product
     ```bash
      near call $ID_ALICE set_products '{"address":"0x1","name":"zapato marca X","price":12345,"stock":4}' --accountId $ID_EVE


  * Alice only can delete product
     ```bash
      near call $ID_ALICE delete_products '{"address":"0x1"}' --accountId $ID_ALICE
     ```


## Code Structure 
```
  .
  |-- Cargo.toml    -- file configuration 
  |-- README.md     -- documentation the project
  |-- bash.sh       -- bash only mac or linux
  `-- src           
      |-- lib.rs    -- source where is smart contract and unit tests
      `-- utils     
          |-- access_control.rs  -- lib over access control
          `-- mod.rs             -- mod que expone access control

  2 directories, 6 files
```
