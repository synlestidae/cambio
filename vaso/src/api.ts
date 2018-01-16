export class Api {
    asyncLogInUser(username: string, password: string): Promise<Session> {
        let login_promise = fetch('/api/users/log_in', {
            method: 'POST'
        });

        return login_promise.then()
    }
}
