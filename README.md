# Management Product
- Simple contract about add, delete, update and get products with stock.
- Add permision Ownable 
- Add AccessControl with roles

## Flow Business
[![](https://mermaid.ink/img/pako:eNp1UsFqwzAM_RXh08qaH8ihkG457NKV9bBLoGi21pk5dmYrhVL675OblmRlNdiI956eJdlHpYMhVapEPz15Tc8WdxHbxoMs1BwiVIAJKmc1wcPr-6p-mw1sh5Gtth16hnXWbFpB4Cl4jpIJBaxjML3mqdkyC5fhY4rVGav3NGBVsVg8rktIxNtuMEh_md1dxpAjphvyfA7h8q7DKjBB2JNUOBeBVAjBuwNo9LKdg5b4K5icOXEFdAyV1pTS0HdwAz4q8qqLYlbljryRlnXoPb-YkZYp2D3K_fWIjWA1gtc20ZitXEXbyYxGlaH_cuu7Y73JuRQhtZ4DNVctxRatkV9yzFCj-ItaalQpocH43ajGn0TXd0YMamPlUVX5iS7RXGHPYXPwWpUce7qKLt_sojr9Anbhx5g)](https://mermaid-js.github.io/mermaid-live-editor/edit/#pako:eNp1UsFqwzAM_RXh08qaH8ihkG457NKV9bBLoGi21pk5dmYrhVL675OblmRlNdiI956eJdlHpYMhVapEPz15Tc8WdxHbxoMs1BwiVIAJKmc1wcPr-6p-mw1sh5Gtth16hnXWbFpB4Cl4jpIJBaxjML3mqdkyC5fhY4rVGav3NGBVsVg8rktIxNtuMEh_md1dxpAjphvyfA7h8q7DKjBB2JNUOBeBVAjBuwNo9LKdg5b4K5icOXEFdAyV1pTS0HdwAz4q8qqLYlbljryRlnXoPb-YkZYp2D3K_fWIjWA1gtc20ZitXEXbyYxGlaH_cuu7Y73JuRQhtZ4DNVctxRatkV9yzFCj-ItaalQpocH43ajGn0TXd0YMamPlUVX5iS7RXGHPYXPwWpUce7qKLt_sojr9Anbhx5g)

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
        $ near create-account $ID_CALLER --masterAccount YYYY.testnet --initialBalance 4
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
