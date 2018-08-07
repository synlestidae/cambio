import {Form} from './form';;
import {Section} from './section';
import {LoadingState} from '../flux/state/loading_state';

export class SingleForm extends Form {
    public loadingState = new LoadingState();

    constructor(sections: Section[], title: string) {
        super(sections, title);
    }
}
