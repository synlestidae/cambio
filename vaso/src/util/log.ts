export interface ILogger {
    info(msg: any): void;
    warn(msg: any): void;
    error(msg: any): void;
}

export class Logger implements ILogger {
    info(msg: any) {
        console.info(msg);
    }

    warn(msg: any) {
        console.warn(msg);
    }

    error(msg: any) {
        console.error(msg);
    }

}
