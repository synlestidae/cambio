import {Session} from './session';

export class Api {
    session: Session|null = null;
    baseUrl = "http://localhost:3000";

    asyncLogInUser(email_address: string, password: string): Promise<Session> {
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

    private makeRequest(url: string, method: string, jsonBody?: any|null): Promise<Response> {
        let urlObj = new URL(this.baseUrl);
        urlObj.pathname = url;
        url = urlObj.toString();
        let headers = {
            'Accept': 'application/json, text/plain, */*',
            'Content-Type': 'application/json'
        };
        if (this.session) {
            headers['Authorization'] = `Bearer ${this.session.session_token}`;
        }
        let params = {
            method: method,
            headers: headers,
            body: undefined
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

        return new Promise(function(res: any, rej: any) {
            fetch(url, params).then(function(response: Response) {
                if (response.status >= 400) {
                    rej(response);
                } else {
                    res(response);
                }
            });
        });
    }
}
