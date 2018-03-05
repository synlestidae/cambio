let Transaction = require('ethereumjs-tx');
let Web3 = require('web3');
let web3 = new Web3;
let ethTransaction = new Transaction(null, 1);
let [fromAddress, toAddress, value, privateKey] = process.argv;
ethTransaction.fromAddress = fromAddress;
ethTransaction.toAddress = toAddress;
ethTransaction.value = web3.toWei(value, 'ether');

// we control the rest
ethTransaction.nonce = 0;
ethTransaction.gasLimit = 21000; //transaction.gasLimit;
ethTransaction.data = '0x0';


var feeCost = ethTransaction.getUpfrontCost();
ethTransaction.gas = feeCost;
ethTransaction.sign(new Buffer(privateKey, 'hex'));
console.log('transaction boi', ethTransaction);
