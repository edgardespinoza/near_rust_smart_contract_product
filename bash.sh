export ID=yyyy.testnet
export ID_ALICE=dww.$ID 
export ID_BOB=dbb.$ID 
export ID_EVE=dee.$ID 

sudo rm -rf ./target
echo '......building project......'
cargo clean & cargo build --target wasm32-unknown-unknown --release

echo '......testing......'
cargo test -- --nocapture

echo $ID
echo 'export ID_ALICE='$ID_ALICE
echo 'export ID_BOB='$ID_BOB
echo 'export ID_EVE='$ID_EVE

echo 'ALICE:' $ID_ALICE
echo 'BOB:'  $ID_BOB
echo 'EVE:'  $ID_EVE

echo '......creation accounts......'
near create-account $ID_ALICE --masterAccount $ID --initialBalance 3
near create-account $ID_BOB --masterAccount $ID --initialBalance 1
near create-account $ID_EVE --masterAccount $ID --initialBalance 2

echo '......deployment to testnet......'
near deploy  --initFunction 'new' --initArgs '{}'  --wasmFile target/wasm32-unknown-unknown/release/sales.wasm --accountId $ID_ALICE

echo '...final...'

