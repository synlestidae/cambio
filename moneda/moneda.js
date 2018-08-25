var Transaction = require('ethereumjs-tx');

 var rawTx = {
   nonce: '0x1',
   gasPrice: '0x2000',
   gasLimit: "0x100000",
   to: '0x0000000000000000000000000000000000000000',
   value: '0x1000',//(1000).toString(16),
   data: '0x'
};
var tx = new Transaction(rawTx);
tx.sign(new Buffer('c0dec0dec0dec0dec0dec0dec0dec0dec0dec0dec0dec0dec0dec0dec0dec0de', 'hex'));
console.log(tx.serialize().toString('hex'));
