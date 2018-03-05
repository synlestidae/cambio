"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
class CasaTransaction {
    constructor() {
        this.id = null;
        this.nonce = null;
        this.gasPrice = null;
        this.gasLimit = null;
        this.toAddress = null;
        this.fromAddress = null;
        this.hash = null;
        this.value = null;
        this.signature = null;
        this.transactionBlockId = null;
        this.uniqueId = null;
    }
    static parseTransaction(json) {
        let transaction = new CasaTransaction();
        transaction.fromAddress = getString(json, 'from_address');
        transaction.toAddress = getString(json, 'to_address');
        transaction.value = getString(json, 'value');
        transaction.uniqueId = getString(json, 'unique_id');
        return transaction;
    }
}
exports.CasaTransaction = CasaTransaction;
function getString(json, prop) {
    if (typeof json[prop] !== 'string') {
        propertyError(prop);
    }
    return json[prop];
}
function propertyError(prop) {
    return `Missing required property: ${prop}`;
}
//# sourceMappingURL=casa_transaction.js.map