import Vue from 'vue';
import { makeHot, reload } from './util/hot-reload';

import {AppState} from './app_state';
import {Api} from './api';
import {buildApp} from './components';


import './sass/main.scss';

window.onload = buildApp;
