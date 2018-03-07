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
ethTransaction.value = value;

// we control the rest
ethTransaction.nonce = 0;
ethTransaction.gasLimit = '0xFFFF'; 
ethTransaction.data = '0x0';
ethTransaction.gasPrice = '0x40000000000';
ethTransaction.gas = '0xF000';

ethTransaction.sign(new Buffer(privateKey, 'hex'));

// now send baby
let serialisedTx = ethTransaction.serialize().toString('hex');
web3.eth.sendSignedTransaction('0x' + serialisedTx, function(err, result) {
    if (err) {
        console.error('Error:', err);
        process.exit(1);
    } else {
        console.log('Success:', result);
        process.exit(0);
    }
});
