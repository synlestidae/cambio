export class CryptoAccount {
    public id: string|null = null; 
    public address: string|null = null; 
    public name = '';

    public static parse(json: any): CryptoAccount {
        let account = new CryptoAccount();
        account.id = json.id || null;
        account.address = json.address.toString();
        account.name = json.name.toString();
        return account;
    }
}
