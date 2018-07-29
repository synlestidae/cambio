import * as React from "react";
import {NavbarComponent} from './navbar_component';
import {ContentComponent} from './content_component';
import {reduce} from './flux/reducer';
import {AppState} from './flux/app_state';
import {Store} from './flux/store';
import {Action} from './flux/action';
import {ActionCreators} from './flux/action_creators';

interface TopLevelProps {
    store: Store,
    actions: ActionCreators
}

export class App extends React.Component<TopLevelProps, AppState> {
    constructor(props: TopLevelProps) {
        super(props);
        this.state = props.store.getState();
    }

    componentDidMount() {
        this.props.store.subscribe((s: AppState) => this.setState(s));
        window.onhashchange = (e: HashChangeEvent) => 
            this.props.actions.changeURL(window.location.hash);
        //this.props.actions.initialise(window.location.hash);
    }

    render() {
        return <div>
            <NavbarComponent></NavbarComponent>
            <ContentComponent state={this.state} actions={this.props.actions}>
            </ContentComponent>
        </div>;
    }
}
