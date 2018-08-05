import {Session} from './session';
import {Account} from './domain/account';
import {Payment} from './domain/Payment';
import {Transaction} from './domain/transaction';
import {Order} from './domain/order';
import {BoardUpdate} from './domain/board_update';
import {OrderRequest} from './domain/order_request';
import {CurrencyCode} from './domain/currency_code';
import {CurrencyDenom} from './domain/currency_denom';
import {PersonalDetails} from './domain/personal_details';
import {RegistrationInfo} from './domain/registration_info';
import {SignupInfo, PersonalInfo, IdentificationInfo} from './flux/state/signup_state';
import {padZeroes} from './pad_zeroes';
import * as bigInt from 'big-integer';

export class Api {
    baseUrl = "http://localhost:3000";
    sessionToken: string|null = null;

    constructor() {
        let item = localStorage.getItem("session_token");
        if (item) {
            this.sessionToken = item;
        }
        console.log('Here is the API for debugging', this);
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
            if (txs.length !== 0) {
                let lastTx = txs[txs.length - 1];
                account.balance = (lastTx.balance / 100).toFixed(2);
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

    public asyncPostOrder(order: OrderRequest): Promise<Order> {
        const WEI_FACTOR = bigInt('1000000000000000000000');
        let milliEther = order.ether * 1000;
        let wei = bigInt(milliEther).multiply(WEI_FACTOR);
        let amountFiat = order.dollars.toString();
        let address = '0x';
        for (let i = 0; i < 40; i++) {
            address = `${address}0`;
        }
        let orderJSON = {
            unique_id: order.uniqueId,
            amount_fiat: order.dollars,
            amount_crypto: `0x${wei.toString(16)}`,
            is_buy: order.isBuy,
            minutes_active: order.minutesActive,
            minutes_to_settle: 24 * 60,
            pledge: '5.00',
            address: address
        };
        return this.makeRequest('/orders/new', 'POST', orderJSON)
            .then((r: Response) => r.json()) 
            .then((json: any) => Order.parse(json));
    }

    public async asyncBuyOrder(order: Order, uniqueId: string): Promise<any> {
        let address = '0x';
        for (let i = 0; i < 40; i++) {
            address = `${address}0`;
        }
        let json = {
            counterparty_order: order.id,
            order_request: {
                unique_id: uniqueId,
                amount_fiat: order.amountFiat,
                amount_crypto: order.amountCrypto,
                is_buy: !order.isBuy,
                minutes_active: 15,
                minutes_to_settle: 24 * 60,
                pledge: '5.00',
                address: address
            }
        };
        let result = await
            this.makeRequest(`orders/${order.isBuy? 'buys' : 'sells'}/complete`, 'POST', json);
        return await result.json();
    }

    public async asyncGetActiveOrders(): Promise<Order[]> {
        let result = await this.makeRequest('/orders/active/', 'GET');
        let body = await result.json();
        if (body instanceof Array) {
            let orders = [];
            for (let order of body) {
                orders.push(Order.parse(order));
            }
            return orders;
        }
        throw new Error(`Unexpected type for asyncGetActiveOrders ${body.constructor.name || typeof body}`);
    }

    public async asyncGetOrderUpdates(lastChecked: Date): Promise<BoardUpdate> {
        const pad = (x: any) => padZeroes(2, x);
        let date = `${lastChecked.getUTCFullYear()}${pad(lastChecked.getUTCMonth() + 1)}${pad(lastChecked.getUTCDate())}`
        let time = `${pad(lastChecked.getUTCHours())}${pad(lastChecked.getUTCMinutes())}${pad(lastChecked.getUTCSeconds())}.${pad(lastChecked.getUTCMilliseconds())}`;
        let lastCheckedString = `${date}${time}`;
        let response = await this.makeRequest('/orders/changed', 'GET', {
            last_change: lastCheckedString
        });
        let body = await response.json();
        return BoardUpdate.parse(body);
    }

    public async asyncGetPersonalDetails(): Promise<PersonalDetails> {
        let result = await this.makeRequest('/users/personal/details', 'GET');
        let body = await result.json();
        return PersonalDetails.parse(body);
    }

    public async asyncPostPersonalDetails(personalDetails: PersonalDetails): Promise<PersonalDetails> {
        let result = await this.makeRequest('/users/personal/details', 'POST', personalDetails);
        let body = await result.json();
        return PersonalDetails.parse(body);
    }

    private async makeRequest(path: string, method: string, jsonBody?: any|null): Promise<Response> {
        let url = this.getURL(path, method, jsonBody);
        let params = this.getParams(method, jsonBody);
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

    private getURL(url: string, method: string, body: any|null) {
        let urlObj = new URL(this.baseUrl);
        //let queryString = '';
        urlObj.pathname = url;
        if (body && method === 'GET') {
            //queryString = '?'
            for (let key in body) {
                urlObj.searchParams.append(key, body[key]);
            //    queryString += `${encodeURIComponent(key)}=${encodeURIComponent(body[key])}`;
            }
        }
        return urlObj.toString();// + queryString;
    }

    private getParams(method: string, body: any|null): RequestInit {
        let headers = new Headers();
        headers.set('Accept', 'application/json, text/plain, */*');
        if (this.sessionToken) {
            headers.set('Authorization', `Bearer ${this.sessionToken}`)
        }
        if (method === 'POST' || method === 'POST') {
            headers.set('Content-Type', 'application/json');
        }
        let params: RequestInit = {
            method: method,
            headers: headers
        };
        if (body && method !== 'GET') {
            let bodyString: string;
            if (typeof body !== 'string') {
                bodyString = JSON.stringify(body, null, 2);
            } else {
                bodyString = body;
            }
            params.body = bodyString;
        }

        (<any>params).credentials = 'include';
        return params;
    }
}
