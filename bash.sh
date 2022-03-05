export ID=eespinor.testnet
export ID_ALICE=ali4.$ID 
export ID_BOB=bob4.$ID 
export ID_EVE=eve4.$ID 

cat banner

rm -rf ./target

echo '......building project......'
read -n1 -r -p "BUILD PROJECT" key
cargo clean & cargo build --target wasm32-unknown-unknown --release

read -n1 -r -p "INIT TESTING..." key
echo '......testing......'
cargo test -- --nocapture

read -n1 -r -p "INIT CONFIGURATION..." key
echo $ID

echo 'ALICE:' $ID_ALICE
echo 'BOB:'  $ID_BOB
echo 'EVE:'  $ID_EVE

echo '......creation accounts......'
near create-account $ID_ALICE --masterAccount $ID --initialBalance 2
near create-account $ID_BOB --masterAccount $ID --initialBalance 0.05
near create-account $ID_EVE --masterAccount $ID --initialBalance 0.3

echo '......deployment to testnet......'
near deploy  --initFunction 'new' --initArgs '{}'  --wasmFile target/wasm32-unknown-unknown/release/sales.wasm --accountId $ID_ALICE

echo '...final...'




echo "
____  _____    _    ______   __
|  _ \| ____|  / \  |  _ \ \ / /
| |_) |  _|   / _ \ | | | \ V / 
|  _ <| |___ / ___ \| |_| || |  
|_| \_\_____/_/   \_\____/ |_|  

please run in your console next commands
"
echo 'export ID_ALICE='$ID_ALICE
echo 'export ID_BOB='$ID_BOB
echo 'export ID_EVE='$ID_EVE