import {Session} from './session';

export class Api {
    session: Session|null = null;

    asyncLogInUser(email_address: string, password: string): Promise<Session> {
        let login_promise = this.makeRequest('http://localhost:3000/users/log_in/', 'PUT', {
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
                document.cookie = `session_token=${session.session_token}`;
                return session;
            });
    }

    private makeRequest(url: string, method: string, jsonBody?: any|null): Promise<Response> {
        let params = {
            method: method,
            headers: {
                'Accept': 'application/json, text/plain, */*',
                'Content-Type': 'application/json'
            },
            body: undefined
        };

        if (jsonBody && typeof jsonBody !== 'string') {
            let bodyString: string;
            bodyString = JSON.stringify(jsonBody);
            params.body = bodyString;
        }

        return fetch(url, params);
    }
}
