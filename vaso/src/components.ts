import {navbarComponent} from './navbar_component';
import {ContentComponent} from './content_component';
import Vue from 'vue';

function buildApp() {
    return new Vue({
        el: '#app',
        components: {
            'content-component': ContentComponent,
            'navbar-component': navbarComponent
        }
    });
}

export {buildApp};
