import {Session} from './session';

export class Api {
    session: Session|null = null;

    asyncLogInUser(email_address: string, password: string): Promise<Session> {
        let login_promise = this.makeRequest('http://localhost:3000/users/log_in/', 'POST', {
            email_address: email_address,
            password: password
        });

        let parent = this;

        return login_promise
            .then((response: Response) => response.json())
            .then((json: any) => {
                return Promise.resolve(Session.parse(json));
            })
            .then((session: Session) => {
                parent.session = session;
                return session;
            });
    }

    private makeRequest(url: string, method: string, jsonBody?: any|null): Promise<Response> {
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

        return fetch(url, params);
    }
}
