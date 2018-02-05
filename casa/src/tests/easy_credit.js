const { Web3 } = require('web3');


let web3 = new Web3();
web3.setProvider(new web3.providers.IpcProvider('/Users/mate/work/cambio/eth_test/data/geth.ipc'));
