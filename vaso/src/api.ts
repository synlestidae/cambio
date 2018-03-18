import {Session} from './session';
import {Account} from './domain/account';
import {Transaction} from './domain/transaction';

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

        console.log('making request', body, params);

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
