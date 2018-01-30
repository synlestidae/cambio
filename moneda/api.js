function handleNewTransaction(transaction, uniqueId) {
    if (!uniqueId) {
        throw new Error('A unique ID is required to process transaction');
    }
    let txPromise = db.getTransactionByUniqueId(unique_id);
    transactionPromise.then(existingTransaction => {
        if (!existingTransaction) {
            return saveTransaction(transaction);
        } else {
            return Promise.reject()
        }
        // return the transaction
    });
}
