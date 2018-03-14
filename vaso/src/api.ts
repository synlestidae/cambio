import {Session} from './session';
import {Account} from './domain/account';

export class Api {
    session: Session|null = null;
    baseUrl = "http://localhost:3000";

    public asyncLogInUser(email_address: string, password: string): Promise<Session> {
        let login_promise = this.makeRequest('/users/log_in/', 'POST', {
            email_address: email_address,
            password: password
        });

        let parent = this;

        return login_promise
            .then((response: Response) => response.json())
            .then((json: any) => {
                return Session.parse(json);
            })
            .then((session: Session) => {
                parent.session = session;
                return session;
            });
    }

    public asyncRegisterUser(email_address: string, password: string): Promise<Session> {
        let that = this;

        return this.makeRequest('/users/register/', 'POST', {
            email_address: email_address,
            password: password
        }).then(() => that.asyncLogInUser(email_address, password));
    }

    public asyncGetAccounts(): Promise<Account[]> {
        return this.makeRequest('accounts/', 'GET')
            .then((r: Response) => r.json())
            .then((accounts: any) => (<Account[]>accounts));
    }

    private makeRequest(url: string, method: string, jsonBody?: any|null): Promise<Response> {
        let urlObj = new URL(this.baseUrl);
        urlObj.pathname = url;
        url = urlObj.toString();
        let headers = {
            'Accept': 'application/json, text/plain, */*',
            'Content-Type': 'application/json'
        };
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
        (<any>params)['mode'] = 'no-cors';

        return fetch(url, params).then(function(response: Response) {
            if (!(response.status >= 400)) {
                return response;
            } else {
                throw response;
            }
        });
    }
}
