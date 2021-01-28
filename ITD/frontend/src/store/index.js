import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    loggedIn: false,
    email: ""
  },
  mutations: {
    logged_in (state){
      state.loggedIn = true;
    },
    logged_out (state){
      state.loggedIn = false;
      state.email = ""
    }
    
  },
  actions: {
  },
  modules: {
  }
})
