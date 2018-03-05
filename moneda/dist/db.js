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
const pg_1 = require("pg");
const casa_transaction_1 = require("./casa_transaction");
const Transaction = require('ethereumjs-tx').Transaction;
class DBService {
    constructor(client) {
        this.client = client;
    }
    getTxByUniqueId(uniqueId) {
        return __awaiter(this, void 0, void 0, function* () {
            let result = yield this.client.query(SELECT_TRANSACTION_SQL, [uniqueId]);
            if (result.rows.length) {
                return casa_transaction_1.CasaTransaction.parseTransaction(result.rows[0]);
            }
            else {
                return null;
            }
        });
    }
    saveTx(transaction) {
        return __awaiter(this, void 0, void 0, function* () {
            throw new Error('Not implemented!');
        });
    }
    updateTxHash(transaction, hash) {
        return __awaiter(this, void 0, void 0, function* () {
            if (transaction.uniqueId !== null) {
                yield this.client.query(UPDATE_HASH_SQL, [transaction.uniqueId, hash]);
                let tx = yield this.getTxByUniqueId(transaction.uniqueId);
                if (tx === null) {
                    throw new Error('Transaction does not exist');
                }
                return tx;
            }
            throw new Error('Transaction should have a uniqueId');
        });
    }
}
exports.DBService = DBService;
function getClient() {
    const client = new pg_1.Client(getConfig());
    return client;
}
function getConfig() {
    return {
        user: 'mate',
        host: 'localhost',
        database: 'test_database_only'
    };
}
const SELECT_TRANSACTION_SQL = 'SELECT * FROM ethereum_outbound_transaction WHERE unique_id = $1';
const INSERT_TRANSACTION_SQL = `
    INSERT INTO ethereum_outbound_transaction
        (gas_price, gas_limit, to_address, from_address, hash, value, signature, unique_id) 
    VALUES($1, $2, $3, $4, $5, $6, $7, $8)`;
const UPDATE_HASH_SQL = 'UPDATE ethereum_outbound_transaction SET hash = $1 WHERE unique_id = $2';
//# sourceMappingURL=db.js.map