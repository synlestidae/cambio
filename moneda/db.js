const { Client } = require('pg');
const { CasaTransaction } = require('./transaction');

function getTxByUniqueId(unique_id) {
    return new Promise(function(res, rej) {
        let client = getClient();
        let result = client.query(SELECT_TRANSACTION_SQL, [unique_id]);
        try {
            if (result.rows.length) {
                return res(CasaTransaction.parseTransaction(result));
            }
        } catch (e) {
            rej(e);
        }
        res(null);
    });
}

function saveTx(transaction) {
    const params = ['gasPrice', 'gasLimit', 'toAddress', 'fromAddress', 'hash', 'value', 'signature', 'uniqueId'];
    let values = params.map(p => transaction[p]);
    return new Promise(function(res, rej) {
        client.query(INSERT_TRANSACTION_SQL, values, function(result, error) {
            if (error) {
                rej(error);
            }
        return getTransactionByUniqueId(transaction.unique_id);
        });
    });
}

function updateTxHash(transaction, hash) {
    return new Promise(function(res, rej) {
        client.query(UPDATE_HASH_SQL, [transaction.unique_id, hash], function(result, error) {
            if (error) {
                rej(error);
            }
            return hash;
        return getTransactionByUniqueId(transaction.unique_id);
        });
    });
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
