import { CasaTransaction } from './casa_transaction';

export class BlockTransaction {
    ethTransaction: any;
    blockHeight: number;

    constructor(ethTransaction: any, blockHeight: number) {
        this.ethTransaction = ethTransaction;
        this.blockHeight = blockHeight;
    }

    confirmsSettlement(casaTransaction: CasaTransaction) {
        throw new Error('Not implemented!');
    }
}
