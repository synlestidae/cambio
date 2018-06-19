export class RegistrationInfo {
    emailAddress: string;
    identifierCode: string;

    constructor(e: string, i: string) {
        this.emailAddress = e;
        this.identifierCode = i;
    }

    public static parse(obj: any): RegistrationInfo {
        let e = obj.email_address;
        let i = obj.identifier_code;
        if (typeof e === 'string' && typeof i  === 'string') {
            return new RegistrationInfo(<string>e, <string>i);
        }
        throw new Error(`Invalid registration values`);
    }
}
