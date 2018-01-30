class CasaTransaction {
    constructor() {
        this.id = null;
        this.nonce = '';
        this.gasPrice = '';
        this.gasLimit = '';
        this.toAddress = '';
        this.fromAddress = '';
        this.hash = '';
        this.value = '';
        this.signature = '';
        this.transactionBlockId = null;
        this.uniqueId = null;
    }

    static parseTransaction(json) {
        let transaction = new CasaTransaction();
        transaction.fromAddress = json.from_address || propertyError('from_address');
        transaction.toAddress = json.to_address || propertyError('to_address');
        transaction.value = json.value || propertyError('value');
        transaction.uniqueId = json.unique_id || propertyError('uniqueId');
        return transaction;
    }
}

function propertyError(prop) {
    return 'Missing required property: ' + prop;
}

module.exports.CasaTransaction = CasaTransaction;
