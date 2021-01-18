import Vue from 'vue'
import App from './App.vue'
import axios from 'axios'
import { BootstrapVue, BootstrapVueIcons } from 'bootstrap-vue'

import Autocomplete from '@trevoreyre/autocomplete-vue'

import './custom.scss';
import './autocompletion-style.css';
import router from './router'

Vue.use(Autocomplete)
Vue.use(BootstrapVue)
Vue.use(BootstrapVueIcons)
Vue.use({
  install (Vue) {
  Vue.prototype.$api = axios.create({
    baseURL: 'https://httpbin.org'
  })
}
})
new Vue({
  el: '#app',
  router,
  render: h => h(App)
})