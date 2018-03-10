#!/usr/bin/env bash

set -e
set -o pipefail

cd ../eth_test/

echo "Removing old data"
rm -rf data/

echo "Creating genesis block"
geth --identity “LocalTestNode” --rpc --rpcport 8081 --rpccorsdomain “*” --datadir data/ --port 30303 --nodiscover --rpcapi db,eth,net,web3,personal --networkid 11 --maxpeers 0 --verbosity 6 init CustomGenesis.json > /dev/null
echo "Starting geth"
geth --identity “LocalTestNode” --rpc --rpcport 8081  --datadir data/ --port 30303 --nodiscover --rpcapi="db,eth,net,web3,personal,web3" --gasprice 4000000000 -networkid 11 --maxpeers 0 --mine --etherbase '0xA990F82d33Fd19C3872dc12c588A66224b9330A6' > /dev/null 2>&1
