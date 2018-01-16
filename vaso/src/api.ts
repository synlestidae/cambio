import {Session} from './session';

export class Api {
    asyncLogInUser(username: string, password: string): Promise<Session> {
        let login_promise = fetch('/api/users/log_in', {
            method: 'POST'
        });

        return login_promise.then((response: Response) => response.json())
            .then((json: any) => {
                return Promise.resolve(Session.parse(json));
            });
    }
}
