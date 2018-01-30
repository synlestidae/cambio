export class CasaTransaction {
    id: string|null = null;
    nonce: string|null = null;
    gasPrice: string|null = null;
    gasLimit: string|null = null;
    toAddress: string|null = null;
    fromAddress: string|null = null;
    hash: string|null = null;
    value: string|null = null;
    signature: string|null = null;
    transactionBlockId: string|null = null;
    uniqueId: string|null = null;

    static parseTransaction(json: any) {
        let transaction = new CasaTransaction();
        transaction.fromAddress = getString(json, 'from_address');
        transaction.toAddress = getString(json, 'to_address');
        transaction.value = getString(json, 'value');
        transaction.uniqueId = getString(json, 'unique_id');
        return transaction;
    }
}

function getString(json: any, prop: string): string {
    if (typeof json[prop] !== 'string') {
        propertyError(prop);
    }
    return <string>json[prop];
}

function propertyError(prop: any) {
    return `Missing required property: ${prop}`;
}
