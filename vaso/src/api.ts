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

    public async asyncPostOrder(order: OrderRequest): Promise<UserOrder> {
        let orderJSON = {
            unique_id: order.unique_id,
            sell_asset_type: order.sell_asset_type,
            sell_asset_denom: order.sell_asset_denom,
            sell_asset_units: order.sell_asset_units,
            buy_asset_type: order.buy_asset_type,
            buy_asset_denom: order.buy_asset_denom,
            buy_asset_units: order.buy_asset_units,
            expires_at: order.expiry.toISOString()
        };
        let orderResult = await this.makeRequest('/orders/new', 'POST', orderJSON); 
        let resultJSON = await orderResult.json();
        return parseUserOrder(resultJSON);
    }

    public async asyncBuyOrder(order: UserOrder, uniqueId: string): Promise<any> {
        let id = parseInt(order.id);
        if (id.toString() !== order.id) {
            throw new Error('Failed to convert order ID to integer');
        }
        let date = new Date();
        date.setMinutes(date.getMinutes() + 10);
        let orderJSON: any = {
            order_id: id,
            order_request: {
                unique_id: uniqueId,
                sell_asset_type: order.buy_asset_type,
                sell_asset_denom: order.buy_asset_denom,
                sell_asset_units: order.buy_asset_units,
                buy_asset_type: order.sell_asset_type,
                buy_asset_denom: order.sell_asset_denom,
                buy_asset_units: order.sell_asset_units,
                expires_at: date
            }
        };
        let result = await this.makeRequest(`/orders/buy`, 'POST', orderJSON);
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
