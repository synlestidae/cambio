export class Transaction {
    public id: string;
    public value: number;
    public balance: number;
    public transactionTime: Date;
    public businessEnds: string;
    public currencyCode: string;
    public note: string;

    private constructor() {
        this.id = '';
        this.value = 0;
        this.balance = 0;
        this.transactionTime = new Date(); 
        this.businessEnds = '';
        this.note = '';
    }

    public static parse(json: any): Transaction {
        console.log('le json', json);
        let transaction = new Transaction();
        transaction.value = json.value;
        transaction.balance = json.balance;
        transaction.transactionTime = 
            json.transaction_time instanceof Date? json.transaction_time : new Date(json.transaction_time);
        transaction.businessEnds = json.business_ends;
        transaction.currencyCode = json.currency_code;
        transaction.note = json.note;
        return transaction;
    }

}
