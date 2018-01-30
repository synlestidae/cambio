class BlockTransaction {
    constructor(ethTransaction, blockHeight) {
        this.ethTransaction = ethTransaction;
        this.blockHeight = blockHeight;
    }

    confirmsSettlement(casaTransaction) {
        throw new Error('Not implemented!');
    }
}

module.exports.BlockTransaction = BlockTransaction;
