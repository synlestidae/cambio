"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
class BlockTransaction {
    constructor(ethTransaction, blockHeight) {
        this.ethTransaction = ethTransaction;
        this.blockHeight = blockHeight;
    }
    confirmsSettlement(casaTransaction) {
        throw new Error('Not implemented!');
    }
}
exports.BlockTransaction = BlockTransaction;
//# sourceMappingURL=block_transaction.js.map