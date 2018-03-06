let Transaction = require('ethereumjs-tx');
let Web3 = require('web3');

if (process.argv.length < 7) {
    console.error('Usage: host fromAddress toAddress value privateKey');
    return;
}

let [_, _1, host, fromAddress, toAddress, value, privateKey] = process.argv;
let ethTransaction = new Transaction(null, 1);

let web3 = new Web3(new Web3.providers.HttpProvider(host));

ethTransaction.fromAddress = fromAddress;
ethTransaction.toAddress = toAddress;
ethTransaction.value = web3.utils.toWei(String(value), 'ether');

// we control the rest
ethTransaction.nonce = 0;
ethTransaction.gasLimit = '21000'; //transaction.gasLimit;
ethTransaction.data = '0x0';
ethTransaction.gasPrice = web3.utils.toWei(String('4000000000'), 'wei');//web3.eth.gasPrice;

var feeCost = ethTransaction.getUpfrontCost();//'0x5208';//String(21000);//;
ethTransaction.gas = '21000';//feeCost;
ethTransaction.sign(new Buffer(privateKey, 'hex'));

console.log('gas go', ethTransaction.gas);

web3.eth.getBlock(0, function(err, result) {
    if (err) {
        console.error('Error:', err);
    } else {
        console.log('The block', result);
    }
});

web3.eth.getBalance(fromAddress, function(err, result) {
    if (err) {
        console.error(err);
    } else {
        console.log('Balance:', result);
    }
});

// now send baby
let serialisedTx = ethTransaction.serialize().toString('hex');
web3.eth.sendSignedTransaction('0x' + serialisedTx, function(err, result) {
    if (err) {
        console.log(err);
    } else {
        console.log(result);
    }
});

