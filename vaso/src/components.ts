import {signupComponent} from './signup_component';
import Vue from 'vue';

function buildApp() {
    console.log('Building your app!');
    return new Vue({
        el: '#app',
        components: {
            'signup-component': signupComponent
        }
    });
}

export {buildApp};
