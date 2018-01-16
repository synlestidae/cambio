import Vue from 'vue';
import {VueConstructor} from 'vue';

const signupComponent = Vue.extend({
    template: '<form>Hi mum!</form>'
    template: `<div class="signup-form">
              <form class="form-signin">
                <div class="form-row">
                  <div>Cambio allows you to buy, sell and trade Ethereum with ease</div>
                </div>
                <div class="form-row">
                  <label for="inputEmail" class="sr-only">Email address</label>
                  <input type="email" id="inputEmail" class="form-control" placeholder="Email address" required="" autofocus="">
                </div>
                <div class="form-row">
                  <label for="inputPassword" class="sr-only">Password</label>
                  <input type="password" id="inputPassword" class="form-control" placeholder="Password" required="">
                </div>
                <div class="form-row">
                  <button class="btn btn-lg btn-primary btn-block" type="submit">Sign in</button>
                </div>
                <div class="form-row">
                  Don't have an account? <a href="javascript: void">Sign up</a> 
                </div>
              </form>`
});

export {signupComponent};
