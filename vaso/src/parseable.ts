export interface Parseable {
    session_token: string,


    parse(source: any): Session
}
