import { Client, QueryResult } from 'pg';
import { CasaTransaction } from './casa_transaction';
const Transaction: any = require('ethereumjs-tx').Transaction;

export class DBService {
    client: Client;

    constructor(client: Client) {
        this.client = client;
    }

    async getTxByUniqueId(uniqueId: string): Promise<CasaTransaction|null> {
        let result = await this.client.query(SELECT_TRANSACTION_SQL, [uniqueId])
        if (result.rows.length) {
            return CasaTransaction.parseTransaction(result.rows[0]);
        } else {
            return null
        }
    }

    async saveTx(transaction: CasaTransaction): Promise<CasaTransaction> {
        throw new Error('Not implemented!');
    }

    async updateTxHash(transaction: CasaTransaction, hash: string): Promise<CasaTransaction> {
        if (transaction.uniqueId !== null) {
            await this.client.query(UPDATE_HASH_SQL, [transaction.uniqueId, hash])
            let tx = await this.getTxByUniqueId(transaction.uniqueId);
            if (tx === null) {
                throw new Error('Transaction does not exist');
            }
            return tx;
        }
        throw new Error('Transaction should have a uniqueId');
    }
}


function getClient() {
    const client = new Client(getConfig());
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
