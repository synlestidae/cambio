import {Session} from './session';
import {Account} from './domain/account';
import {Payment} from './domain/Payment';
import {Transaction} from './domain/transaction';
import {UserOrder} from './domain/user_order';
import {OrderRequest} from './domain/order_request';
import {CurrencyCode} from './domain/currency_code';
import {CurrencyDenom} from './domain/currency_denom';
import {RegistrationInfo} from './domain/registration_info';
import {SignupInfo, PersonalInfo, IdentificationInfo} from './flux/state/signup_state';
import * as bigInt from 'big-integer';

export class Api {
    baseUrl = "http://localhost:3000";
    sessionToken: string|null = null;

    constructor() {
        let item = localStorage.getItem("session_token");
        if (item) {
            this.sessionToken = item;
        }
    }

    public asyncLogInUser(email_address: string, password: string): Promise<void> {
        let login_promise = this.makeRequest('/users/log_in/', 'POST', {
            email_address: email_address,
            password: password
        });

        let parent = this;

        return login_promise.then((r: Response) => r.json())
            .then((session_json: any) => {
                parent.sessionToken = session_json.session_token;
                localStorage.setItem("session_token", parent.sessionToken);
            });
    }

    public asyncRegisterUser(email_address: string, password: string): Promise<RegistrationInfo> {
        return this.makeRequest('/users/register/', 'POST', {
            email_address: email_address,
            password: password
        })
        .then((r: Response) => r.json())
        .then((result: any) => RegistrationInfo.parse(result));
    }


    public async asyncResendEmail(email: string, identifierCode: string) : Promise<RegistrationInfo> {
        return this.makeRequest('/users/register/new_confirmation_email', 'POST', {
            email_address: email,
            identifierCode: identifierCode 
        })
        .then((r: Response) => r.json())
        .then((result: any) => RegistrationInfo.parse(result));
    }


    public asyncConfirmRegistration(confirmationCode: string,
        identifierCode: string,
        signupInfo: SignupInfo,
        personalInfo: PersonalInfo,
        identificationInfo: IdentificationInfo): Promise<void> {
        return this.makeRequest('/users/confirm/', 'POST', {
            email_address: signupInfo.email_address,
            eth_account_password: signupInfo.password,
            confirmation_code: confirmationCode,
            identifier_code: identifierCode,
            personal_details: {
                first_names: personalInfo.first_names,
                family_name: personalInfo.family_name,
                address_line_1: personalInfo.address_line_1,
                address_line_2: personalInfo.address_line_2,
                post_code: personalInfo.post_code,
                city: personalInfo.city,
                dob: personalInfo.dob.getDateString(),
                country: 'NEW ZEALAND',
                id_type: identificationInfo.id_type,
                id_number: identificationInfo.id_number
            }
        }).then((r: Response) => r.json());
    }

    public async asyncGetAccounts(): Promise<Account[]> {
        let accountJSON = await this.makeRequest('/accounts/', 'GET')
            .then((r: Response) => r.json());

        let accounts: Account[] = [];

        for (let a of accountJSON) {
            let account: Account = Account.parse(a);
            let txs = await this.asyncGetAccountTransactions(account.id);
            if (txs.length === 0) {
                let lastTx = txs[txs.length - 1];
                account.balance = (lastTx.balance / 100).toString();
            }
            account.transactions = txs;
            accounts.push(account);
        }

        return accounts;
    }

    public asyncGetAccountTransactions(accountId: string): Promise<Transaction[]> {
        return this.makeRequest(`/accounts/${accountId}/transactions/`, 'GET')
            .then((r: Response) => r.json())
            .then((transactions: any) => (<Transaction[]>transactions));
    }

    public async asyncPostPayment(payment: Payment): Promise<Payment> {
        let result = await this.makeRequest('/payment', 'POST', payment);
        let body = await result.json();
        return <Payment>body;
    }

    public asyncPostOrder(order: OrderRequest): Promise<UserOrder> {
        const WEI_FACTOR = bigInt('1000000000000000000');
        console.log('WEI', WEI_FACTOR);
        let isBuy = order.buy_asset_type === 'ETH';
        let amountFiat: string;
        if (isBuy) {
            amountFiat = order.sell_asset_units.toString();
        } else {
            amountFiat = order.buy_asset_units.toString();
        }
        let ether = isBuy? order.buy_asset_units : order.sell_asset_units;
        let milliEther = ether * 1000;
        let orderJSON: any = {
            unique_id: order.unique_id,
            amount_fiat: amountFiat,
            amount_crypto: `0x${bigInt(milliEther).multiply(WEI_FACTOR).toString(16)}`,
            is_buy: isBuy,
            minutes_active: 15, // TODO These numbers don't even make sense
            max_wei: isBuy? null : '0x0'
        };
        console.log('fucking json', orderJSON);
        return this.makeRequest('/orders/new', 'POST', orderJSON)
            .then((r: Response) => r.json()) 
            .then((json: any) => parseUserOrder(json));
    }

    public async asyncBuyOrder(order: UserOrder, uniqueId: string): Promise<any> {
        throw new Error('Not implemented!');
    }

    public async asyncGetActiveOrders(): Promise<UserOrder[]> {
        let result = await this.makeRequest('/orders/active/', 'GET');
        let body = await result.json();
        if (body instanceof Array) {
            let orders = [];
            for (let order of body) {
                let userOrder = parseUserOrder(order);
                orders.push(userOrder);
            }
            return orders;
        }
        throw new Error(`Unexpected type for asyncGetActiveOrders ${body.constructor.name || typeof body}`);
    }

    private async makeRequest(url: string, method: string, jsonBody?: any|null): Promise<Response> {
        let urlObj = new URL(this.baseUrl);
        urlObj.pathname = url;
        url = urlObj.toString();
        let headers = new Headers();
        headers.set('Accept', 'application/json, text/plain, */*');
        headers.set('Content-Type', 'application/json');
        if (this.sessionToken) {
            headers.set('Authorization', `Bearer ${this.sessionToken}`)
        }
        let body: string|null = null;
        let params = {
            method: method,
            headers: headers,
            body: body
        };

        if (jsonBody) {
            let bodyString: string;
            if (typeof jsonBody !== 'string') {
                bodyString = JSON.stringify(jsonBody);
            } else {
                bodyString = jsonBody;
            }
            params.body = bodyString;
        }

        (<any>params).credentials = 'include';

        let response: Response;
        try {
            response = await fetch(url, params);
        } catch (e) {
            return Promise.reject(e);
        }
        if (!(response.status >= 400)) {
            return response;
        } else {
            throw response;
        }
    }
}

function parseUserOrder(order: any) {
    return new UserOrder(
        <string>order.id.toString(),
        new Date(order.expires_at),
        <string>order.status,
        <CurrencyCode>order.sell_asset_type,
        <CurrencyDenom>order.sell_asset_denom,
        <number>order.sell_asset_units,
        <CurrencyCode>order.buy_asset_type,
        <CurrencyDenom>order.buy_asset_denom,
        <number>order.buy_asset_units
    );
}
