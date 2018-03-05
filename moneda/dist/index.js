let { Transaction } = require('ethereumjs-tx');
console.log('poop', process.argv);
let ethTransaction = new Transaction(null, 1);
/*ethTransaction.toAddress = transaction.toAddress;
ethTransaction.fromAddress = transaction.fromAddress;
ethTransaction.value = transaction.value;

// we control the rest
ethTransaction.nonce = 0;
ethTransaction.gasLimit = 21000; //transaction.gasLimit;
ethTransaction.data = '0x0';

var feeCost = ethTransaction.getUpfrontCost();
ethTransaction.gas = feeCost;

ethTransaction.sign(new Buffer(privateKey, 'hex'));*/
//# sourceMappingURL=index.js.map