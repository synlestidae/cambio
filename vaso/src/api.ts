import {Session} from './session';
import {Account} from './domain/account';
import {Payment} from './domain/Payment';
import {Transaction} from './domain/transaction';
import {UserOrder} from './domain/user_order';
import {CurrencyCode} from './domain/currency_code';
import {CurrencyDenom} from './domain/currency_denom';

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

    public asyncRegisterUser(email_address: string, password: string): Promise<void> {
        let that = this;

        return this.makeRequest('/users/register/', 'POST', {
            email_address: email_address,
            password: password
        }).then(() => that.asyncLogInUser(email_address, password));
    }

    public asyncGetAccounts(): Promise<Account[]> {
        return this.makeRequest('/accounts/', 'GET')
            .then((r: Response) => r.json())
            .then((accounts: any) => (<Account[]>accounts));
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

    public async asyncGetActiveOrders(): Promise<UserOrder[]> {
        let result = await this.makeRequest('/orders/active/', 'GET');
        let body = await result.json();
        if (body instanceof Array) {
            let orders = [];
            for (let order of body) {
                let userOrder = new UserOrder(
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
                orders.push(userOrder);
            }
            return orders;
        }
        throw new Error(`Unexpected type for asyncGetActiveOrders ${body.constructor.name || typeof body}`);
    }

    private makeRequest(url: string, method: string, jsonBody?: any|null): Promise<Response> {
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

        return fetch(url, params).then(function(response: Response) {
            if (!(response.status >= 400)) {
                return response;
            } else {
                throw response;
            }
        });
    }
}
