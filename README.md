# Management Product
Simple contract about add, delete, update and get products with stock.


## Flow Business
[![](https://mermaid.ink/img/pako:eNp1kN1KA0EMhV8l5Epx-wJ7UdjV3tbFXnizIHEmtoPzs85khFL67mbdCip0IDCc852Q5IQmWcYWC39UjoYfHO0zhTGCPjKSMnRABTrvDMPN4_N283S7uBNlccZNFAWGmdkFVeA-RcmahBUMOdlq5Hezfgb79Lpo3Wq9vhtaKCwv0wKXv87-qmPZs_A_s78a2yZhSJ-sIzQK6AiQoj-CoajlPQSWQ7Jz8juADQbOgZzV65xmaUQ5cOARW_1ayu8jjvGsXJ0sCW-s0wWxfSNfuEGqknbHaLCVXPkHupz3Qp2_AEttfOA)](https://mermaid-js.github.io/mermaid-live-editor/edit/#pako:eNp1kN1KA0EMhV8l5Epx-wJ7UdjV3tbFXnizIHEmtoPzs85khFL67mbdCip0IDCc852Q5IQmWcYWC39UjoYfHO0zhTGCPjKSMnRABTrvDMPN4_N283S7uBNlccZNFAWGmdkFVeA-RcmahBUMOdlq5Hezfgb79Lpo3Wq9vhtaKCwv0wKXv87-qmPZs_A_s78a2yZhSJ-sIzQK6AiQoj-CoajlPQSWQ7Jz8juADQbOgZzV65xmaUQ5cOARW_1ayu8jjvGsXJ0sCW-s0wWxfSNfuEGqknbHaLCVXPkHupz3Qp2_AEttfOA)

## Setting up
- install environment
  ```bash
   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   $ rustup target add wasm32-unknown-unknown
   $ npm install -g near-cli
   ```

- setting ID OWNER is who deployment the contract, change YYYY.testnet  for your account
    ```
    $   export ID_OWNER=ds4.YYYY.testnet 
    ```
- setting ID CALLER is who calling methods from contract, change YYYY.testnet  for your account
    ```
    $   export ID_CALLER=ds0.YYYY.testnet 
    ```
    
- to login and create new account , change YYYY.testnet  for your account
    ```bash
        $ near login 
        $ near create-account $ID_CALLER --masterAccount YYYY.testnet --initialBalance 1
    ```

- to clean the folder target
  ```bash
   $ cargo clean 
  ```

- to build contract 
  ```bash
   $ cargo build --target wasm32-unknown-unknown --release
  ```
- to test contract 
    ```bash
    $  cargo test -- --nocapture
    ```

- to deploy contract 
    ```bash
    $ near deploy  --initFunction 'new' --initArgs '{}'  --wasmFile target/wasm32-unknown-unknown/release/sales.wasm --accountId $ID_OWNER
    ``` 

- call smart contracts
  * add product
    ```bash
      near call $ID_OWNER set_products '{"address":"0x1","price":12345,"stock":5}' --accountId $ID_OWNER
     ```

  * get product
    ```bash
      near call $ID_OWNER get_products '{"address":"0x1"}' --accountId $ID_OWNER
     ```

  * update product
     ```bash
      near call $ID_OWNER set_products '{"address":"0x1","price":12345,"stock":4}' --accountId $ID_CALLER
     ```

  * delete product
     ```bash
      near call $ID_OWNER delete_products '{"address":"0x1"}' --accountId $ID_OWNER
     ```


## Code Structure 
```
  .
  ├── Cargo.toml    -- file configuration 
  ├── README.md     -- documentation
  └── src           -- directory source
      └── lib.rs    -- file where is smart contract and unit tests
```
