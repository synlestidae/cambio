import * as React from 'react';

interface EditBoxProps {
    value: string,
    onDone: (value: string) => void
}

interface EditBoxState {
    isEditing: boolean, 
    editValue: string
}

export class EditBox extends React.Component<EditBoxProps, EditBoxState> {
    render() {
        if (this.state && this.state.isEditing) {
            return <span>
                <input type="text" className="form-control little-input" value={this.state.editValue} onChange={(e: any) => this.onEdit(e.target.value)} />
                <i className="fas fa-check clickable" onClick={(e: any) => this.onDone()}></i>
                <i className="fas fa-times clickable" onClick={(e: any) => this.onCancel()} ></i>
                </span>;
        }
        return <span>
            <span>{this.props.value} </span>
            <i className="far fa-edit clickable" onClick={() => this.onEdit(this.props.value)}></i>
        </span>;
    }

    private onEdit(value: string) {
        this.setState({editValue: value, isEditing: true});
    }

    private onDone() {
        const newVal = this.state.editValue;
        this.setState({isEditing: false, editValue: this.props.value});
        this.props.onDone(newVal);
    }

    private onCancel() {
        this.setState({isEditing: false});
    }
}
