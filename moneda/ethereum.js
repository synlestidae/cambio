const { BlockTransaction } = require('./block_transaction');

class EthereumService {
    constructor(web3) {
        if (!web3) {
            throw new Error('Cannot create EthereumService without web3 instance');
        }
        this.web3 = web3;
    }

    asyncSendTransaction(ethTransaction) {
        return new Promise((res, rej) => {
            let rawTx = ethTransaction.serialize().toString('hex');
            try {
                web3.eth.sendRawTransaction('0x' + rawTx, function(err, hash) {
                    if (err) {
                        rej(err);
                    }
                    res(hash);
                });
            } catch (e) {
                rej(e);
            }
        });
    }

    asyncCheckTransaction(ethTransaction) {
        let txPromise = this.asyncGetTransaction(ethTransaction.hash);
        let heightPromise = this.asyncGetBlockNumber();
        let promise = Promise.all([txPromise, heightPromise]);
        promise.then(resultArray => {
            let tx = resultArray[0];
            let confirmations = blockNumber - tx.blockNumber;
            return new BlockTransaction(tx, blockNumber);
        });
    }

    asyncGetTransaction(hash) {
        return new Promise(function(res, rej) {
            try {
                this.web3.getTransaction(ethTransaction.hash, function(tx) {
                    res(tx);
                });
            } catch (e) {
                rej(e);
            }
        });
    }

    asyncGetBlockNumber() {
        return new Promise(function(res, rej) {
            try {
                web3.eth.getBlockNumber((err, result) => err? rej(err) : res(result));
            } catch (e) {
                rej(e);
            }
        });
    }
}
