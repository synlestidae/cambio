const http = require('http');
const Transaction = require('ethereumjs-tx');
const express = require('express');
const { CasaTransaction } = require('./transaction');
const db = require('./db');

const app = express();

app.use(require('body-parser').json());

app.post('/transaction', function(request, response){
    let transaction = CasaTransaction.parseTransaction(request.body);
    let privateKey = request.body.private_key;
    let ethService = new EthereumService(getWeb3());

    // user only needs to provide these, and private key
    let ethTransaction = new Transaction(null, 1);
    ethTransaction.toAddress = transaction.toAddress;
    ethTransaction.fromAddress = transaction.fromAddress;
    ethTransaction.value = transaction.value;

    // we control the rest
    ethTransaction.nonce = 0;
    ethTransaction.gasLimit = 21000; //transaction.gasLimit;
    ethTransaction.data = '0x0';

    var feeCost = ethTransaction.getUpfrontCost();
    ethTransaction.gas = feeCost;

    ethTransaction.sign(new Buffer(privateKey, 'hex'));
    if (!(ethTransaction.verifySignature() && ethTransaction.validate())) {
        throw new Error('Transaction has invalid signature!');
    }
    db.saveTx(ethTransaction, transaction.uniqueId).then(() => {
        return ethService.asyncSendTransaction(ethTransaction);
    }).then((hash) => {
        return db.updateTxHash(transaction, hash);
    }).then(() => {
        throw new Error('Not implemented!');
    });
});

app.get('/info', function(request, response) {
    throw new Error('Not implemented!');
});

const PROVIDER_ADDRESS = 'http://localhost:8545';

function getWeb3() {
    return new Web3(new Web3.providers.HttpProvider(PROVIDER_ADDRESS));
}


app.listen(3000);