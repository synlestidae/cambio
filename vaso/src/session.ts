import {Parseable} from './parseable';

export class Session implements Parseable{
    parse(source: any): Session {
        if (typeof source === 'string') {
            source = JSON.parse(source);
        }

    }
}
