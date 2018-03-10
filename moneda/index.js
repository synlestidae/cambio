let Transaction = require('ethereumjs-tx');
let Web3 = require('web3');
let net = require('net');

if (process.argv.length < 7) {
    console.error('Usage: host fromAddress toAddress value privateKey');
    return;
}

let [_, _1, ipcPath, fromAddress, toAddress, value, privateKey] = process.argv;
let ethTransaction = new Transaction(null, 1);

let web3 = new Web3(new Web3.providers.IpcProvider(ipcPath, net));

ethTransaction.from = fromAddress;
console.log('getting the goods', fromAddress, ethTransaction.fromAddress);
ethTransaction.to = toAddress;
ethTransaction.value = value;

// we control the rest
ethTransaction.nonce = 0;
ethTransaction.data = '';
ethTransaction.gas = (21000).toString(16);//'0x5208';
ethTransaction.gasPrice = (4000000000).toString(16); //'0x40000000000';
ethTransaction.gasLimit = (25000).toString(16); 


var newTxHash = null;

// now send baby
web3.eth.getTransactionCount(fromAddress, function(err, result) {
    if (err) {
        console.log('Error getting nonce: ', err);
        process.exit(1);
        return;
    }
    ethTransaction.nonce = result + 1;
    ethTransaction.sign(new Buffer(privateKey, 'hex'));
    let serialisedTx = ethTransaction.serialize().toString('hex');
    var gas = web3.eth.getGasPrice();
    gas.then(g => console.log('f', g));
    console.log('gas', ethTransaction.gas, ethTransaction.value);
    web3.eth.sendSignedTransaction('0x' + serialisedTx, function(err, result) {
        if (err) {
            console.error('Error transferring ', value, 'from', fromAddress, 'to', toAddress);
            console.error(err);
            //process.exit(1);
        } else {
            newTxHash = result;
            console.log('Sent:', newTxHash);
        }
    });
});

web3.eth.subscribe('newBlockHeaders', onShitHappen)

function onShitHappen(error, result) {
    try {
        if (newTxHash === null) {
            return;
        }
        if (!error && newTxHash) {
            let blockPromise = web3.eth.getBlock(result.number);
            blockPromise.then(function(block) {
                if (block.transactions && block.transactions.length > 0) {
                    console.log('Flock', block);
                    if (block.transactions.indexOf(newTxHash)) {
                        console.log('Success:', newTxHash);
                        process.exit(0);
                    }
                }
            });
        }
    } catch (e) {
        console.error(e);
        //process.exit(1);
    }
}

web3.eth.subscribe('logs', {}, console.log);
