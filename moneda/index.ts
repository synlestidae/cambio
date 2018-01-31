//const http = require('http');
const Transaction: any = require('ethereumjs-tx');
const Web3 = require('web3');
import * as express from 'express';
import * as bodyParser from "body-parser";
import { NextFunction, Request, Response } from "express";
import { CasaTransaction } from './casa_transaction';
import { DBService } from './db';
import { EthereumService } from './ethereum';

const app = express();

app.use(bodyParser.json());

app.post('/transaction', async function(request: Request, response: Response) {
    response.setHeader('Content-Type', 'application/json');
    let transaction = CasaTransaction.parseTransaction(request.body);
    let privateKey: string = request.body.private_key;
    let ethService = new EthereumService(getWeb3());
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
    try {
        let result = await ethService.sendTransaction(ethTransaction);
        response.send(JSON.stringify({ transaction_result : result }));
    } catch (e) {
        response.send(JSON.stringify({ error_message : e.message }));
    }
});


const PROVIDER_ADDRESS = 'http://localhost:8545';

function getWeb3() {
    return new Web3(new Web3.providers.HttpProvider(PROVIDER_ADDRESS));
}

console.log('Starting up...');

app.listen(3000);

console.log('Ready!');
