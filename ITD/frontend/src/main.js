import Vue from 'vue'
import App from './App.vue'
import axios from 'axios'
import { BootstrapVue, BootstrapVueIcons } from 'bootstrap-vue'

import Autocomplete from '@trevoreyre/autocomplete-vue'

import './custom.scss';
import './autocompletion-style.css';
import router from './router'
import store from './store'
import VueQriously from 'vue-qriously'
import './registerServiceWorker'


Vue.use(Autocomplete)
Vue.use(BootstrapVue)
Vue.use(BootstrapVueIcons)
Vue.use(VueQriously)
Vue.use({
  install (Vue) {
  Vue.prototype.$api = axios.create({
    baseURL: ('VUE_APP_API_BASE_URL' in process.env)?process.env.VUE_APP_API_BASE_URL:'/api',
    withCredentials: true
  })
}
})
new Vue({
  el: '#app',
  router,
  store,
  render: h => h(App)
})