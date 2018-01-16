import {SignupComponent} from './signup_component';
import {navbarComponent} from './navbar_component';
import Vue from 'vue';

function buildApp() {
    console.log('Building your app!');
    return new Vue({
        el: '#app',
        components: {
            'signup-component': SignupComponent,
            'navbar-component': navbarComponent 
        }
    });
}

export {buildApp};
