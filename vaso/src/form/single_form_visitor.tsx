import {ReactFormVisitor} from './react_form_visitor';
import * as React from 'react';

export class SingleFormVisitor extends ReactFormVisitor {
    public visitOnCancel(onCancel: () => void) {
        this.components.push(<input type="button" value="Cancel" onClick={onCancel} />);
    }
}
