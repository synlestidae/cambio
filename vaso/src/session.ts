export class Session {
    session_token: string;
    expires_at: Date;

    constructor(session_token: string, expires_at: Date) {
        this.session_token = session_token;
        this.expires_at = expires_at;
    }

    static parse(source: any): Session {
        if (typeof source === 'string') {
            source = JSON.parse(source);
        }
        return new Session(source.session_token, source.expires_at);
    }
}
