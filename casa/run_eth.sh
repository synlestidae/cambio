#!/usr/bin/env bash

set -e
set -o pipefail

cd ../eth_test/

echo "Removing old data"
rm -rf data/

echo "Creating genesis block"
geth --identity "LocalTestNode" --rpc --rpcport 8081 --rpccorsdomain "localhost" --datadir data/ --port 30303 --nodiscover --rpcapi ipc,db,eth,net,web3,personal --networkid 11 --maxpeers 0 --verbosity 6 init CustomGenesis.json #> /dev/null
geth --identity "LocalTestNode" --rpc --ipcpath="./geth.ipc" --rpcaddr 0.0.0.0 --rpcport 8081 --rpccorsdomain localhost --datadir data/ --port 30303 --nodiscover --rpcapi ipc,db,eth,net,web3,personal --gasprice 4000000000 -networkid 11 --maxpeers 0 --mine --etherbase '0xA990F82d33Fd19C3872dc12c588A66224b9330A6'
