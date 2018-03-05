"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : new P(function (resolve) { resolve(result.value); }).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const block_transaction_1 = require("./block_transaction");
class EthereumService {
    constructor(web3) {
        if (!web3) {
            throw new Error('Cannot create EthereumService without web3 instance');
        }
        this.web3 = web3;
    }
    sendTransaction(ethTransaction) {
        return __awaiter(this, void 0, void 0, function* () {
            let web3 = this.web3;
            return new Promise(function (res, rej) {
                return __awaiter(this, void 0, void 0, function* () {
                    let rawTx = ethTransaction.serialize().toString('hex');
                    try {
                        web3.eth.sendSignedTransaction(ethTransaction, function (err, hash) {
                            if (err) {
                                rej(err);
                            }
                            res(hash);
                        });
                    }
                    catch (e) {
                        rej(e);
                    }
                });
            });
        });
    }
    getBlockTransaction(ethTransaction) {
        return __awaiter(this, void 0, void 0, function* () {
            let tx = yield this.getTransaction(ethTransaction.hash);
            let blockNumber = yield this.getBlockNumber();
            let confirmations = blockNumber - tx.blockNumber;
            return new block_transaction_1.BlockTransaction(tx, blockNumber);
        });
    }
    getTransaction(hash) {
        return __awaiter(this, void 0, void 0, function* () {
            return new Promise(function (res, rej) {
                this.web3.getTransaction(hash, function (err, hash) {
                    if (err) {
                        rej(err);
                    }
                    res(hash);
                });
            });
        });
    }
    getBlockNumber() {
        return new Promise(function (res, rej) {
            this.web3.eth.getBlockNumber(function (err, blockNum) {
                if (err) {
                    rej(err);
                }
                res(blockNum);
            });
        });
    }
}
exports.EthereumService = EthereumService;
//# sourceMappingURL=ethereum.js.map