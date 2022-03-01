# Management Product
Simple contract about add, delete, update and get products with stock.

## Setting up
- install environment
  ```bash
   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   $ rustup target add wasm32-unknown-unknown
   $ npm install -g near-cli
   ```

   change  YYY.testnet for your account
- to login and create new account 
    ```bash
        $ near login 
        $ near create-account de4.YYYY.testnet --masterAccount YYYY.testnet --initialBalance 10
    ```

- setting ID
    ```
    $   ID=de4.YYYY.testnet
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
    $ near deploy  --wasmFile target/wasm32-unknown-unknown/release/sales.wasm --accountId $ID
    ``` 

- call smart contracts
  * add product
    ```bash
      near call $ID set_products '{"address":"0x1","price":12345,"stock":5}' --accountId $ID
     ```

  * get product
    ```bash
      near call $ID get_products '{"address":"0x1"}' --accountId $ID
     ```

  * update product
     ```bash
      near call $ID set_products '{"address":"0x1","price":12345,"stock":4}' --accountId $ID
     ```

  * delete product
     ```bash
      near call $ID delete_products '{"address":"0x1"}' --accountId $ID
     ```

## Code Structure 
```
  .
  ├── Cargo.toml    -- file configuration 
  ├── README.md     -- documentation
  └── src           -- directory source
      └── lib.rs    -- file where is smart contract and unit tests
```