import Vue from 'vue';
import {VueConstructor} from 'vue';

const navbarComponent = Vue.extend({
    template: `
        <div id="top-nav">
          <div class="nav-logo">
            CAMBIO Ltd.
          </div>
          <div class="nav-element clickable" v-for="nav in links">
            {{ nav.label }}
          </div>
        </div>`,
    data: function() {
        return {
            links: ['Home', 'Board', 'Buy', 'Sell', 'My account'].map(label => ({label: label}))
        };
    }
});

export {navbarComponent};
