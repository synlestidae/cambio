import {Session} from './session';

export class Api {
    asyncLogInUser(email_address: string, password: string): Promise<Session> {
        let login_promise = fetch('http://localhost:3000/users/register/', {
            method: 'PUT',
            headers: {
                'Accept': 'application/json, text/plain, */*',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({email_address: email_address, password: password})
        });

        return login_promise.then((response: Response) => response.json())
            .then((json: any) => {
                return Promise.resolve(Session.parse(json));
            });
    }
}
