#!/usr/bin/env bash

set -e
set -o pipefail

cd ../eth_test/

echo "Removing old data"
rm -rf data/

echo "Creating genesis block"
geth --identity “LocalTestNode” --rpc --rpcport 8080 --rpccorsdomain “*” --datadir data/ --port 30303 --nodiscover --rpcapi db,eth,net,web3,personal --networkid 1999 --maxpeers 0 --verbosity 6 init CustomGenesis.json
echo "Starting geth"
geth --identity “LocalTestNode” --rpc --datadir data/ --port 30303 --nodiscover --rpcapi ipc --networkid 1999 --maxpeers 0 
#--mine --etherbase '0x9f23fedfa2ce3a321f20f6a95d0c2cbabb5876dd' 
