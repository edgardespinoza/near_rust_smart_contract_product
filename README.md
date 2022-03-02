# Management Product
Simple contract about add, delete, update and get products with stock.

## Setting up
- install environment
  ```bash
   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   $ rustup target add wasm32-unknown-unknown
   $ npm install -g near-cli
   ```

- setting ID OWNER is who deployment the contract, change YYYY.testnet  for your account
    ```
    $   export ID_OWNER=ds2.YYYY.testnet 
    ```
- setting ID CALLER is who calling methods from contract, change YYYY.testnet  for your account
    ```
    $   export ID_CALLER=ds2.YYYY.testnet 
    ```
    
- to login and create new account , change YYYY.testnet  for your account
    ```bash
        $ near login 
        $ near create-account $ID_OWNER --masterAccount YYYY.testnet --initialBalance 4
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
      near call $ID_OWNER set_products '{"address":"0x1","price":12345,"stock":5}' --accountId $ID_CALLER
     ```

  * get product
    ```bash
      near call $ID_OWNER get_products '{"address":"0x1"}' --accountId $ID_CALLER
     ```

  * update product
     ```bash
      near call $ID_OWNER set_products '{"address":"0x1","price":12345,"stock":4}' --accountId $ID_CALLER
     ```

  * delete product
     ```bash
      near call $ID_OWNER delete_products '{"address":"0x1"}' --accountId $ID_CALLER
     ```


## Code Structure 
```
  .
  ├── Cargo.toml    -- file configuration 
  ├── README.md     -- documentation
  └── src           -- directory source
      └── lib.rs    -- file where is smart contract and unit tests
```

## Flow Business
[![](https://mermaid.ink/img/pako:eNqVkM0KwjAQhF-l7EmxvkAOBX96VbEHL4ESk7UGm6Qmm4OUvruR1osH0TksO_DNwk4P0ikEBgHvEa3ErRaNF4bbLGmcq1ZLzGb70648zpdFsag29cE7FSWxLCDV3WjCD3jzH66wRcKPxNqdl9_uQg4GvRFapb_6V4YDXdEgB5ZWJfyNA7dD4mKnBGGpNDkP7CLagDmISK56WAmMfMQ3NBUzUcMTc2BoKw)](https://mermaid-js.github.io/mermaid-live-editor/edit#pako:eNqVkM0KwjAQhF-l7EmxvkAOBX96VbEHL4ESk7UGm6Qmm4OUvruR1osH0TksO_DNwk4P0ikEBgHvEa3ErRaNF4bbLGmcq1ZLzGb70648zpdFsag29cE7FSWxLCDV3WjCD3jzH66wRcKPxNqdl9_uQg4GvRFapb_6V4YDXdEgB5ZWJfyNA7dD4mKnBGGpNDkP7CLagDmISK56WAmMfMQ3NBUzUcMTc2BoKw)