import {Order} from './order';

export class UserSettlement {
    sourceOrder: Order;
    settlementStatus: string;
    fromAddress: string;
    toAddress: string;
    value: string;
    dueOnBlockchainAt: Date;

    private constructor() {
    }

    public static parse(json: any): UserSettlement {
        throw new Error();
    }
}
