import {ReactFormVisitor} from './react_form_visitor';
import {LoadingState} from '../flux/state/loading_state';
import * as React from 'react';

export class SingleFormVisitor extends ReactFormVisitor {
    public visitOnCancel(onCancel: () => void) {
        this.components.push(<input type="button" value="Cancel" onClick={onCancel} />);
    }
    
    public visitLoadingState(loadingState: LoadingState) {
        if (loadingState.name === 'Error') {
            let message: string;
            if (loadingState.message) {
                message = `There was an error: ${loadingState.message}`;
            } else {
                message = 'There was an unknown error. Maybe try again.';
            }
            this.components.push(<p className="error-text">{message}</p>);
        }
    }
}
