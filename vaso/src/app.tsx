import * as React from "react";
import {NavbarComponent} from './navbar_component';
import {ContentComponent} from './content_component';
import {reduce} from './flux/reducer';
import {AppState} from './flux/app_state';
import {Store} from './flux/store';
import {Action} from './flux/action';

interface TopLevelProps {
    dispatch: (action: Action) => void,
    store: Store
}

export class App extends React.Component<TopLevelProps, AppState> {
    constructor(props: TopLevelProps) {
        super(props);
        this.state = props.store.getState();
    }

    componentDidMount() {
        this.props.store.subscribe((s: AppState) => this.setState(s));
    }

    render() {
        return <div>
            <NavbarComponent></NavbarComponent>
            <ContentComponent></ContentComponent>
        </div>;
    }
}
