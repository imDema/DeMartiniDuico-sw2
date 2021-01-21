import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    logged_in: false
  },
  mutations: {
    logged_in (state){
      state.logged_in = true;
    },
    logged_out (state){
      state.logged_in = false;
    }
    
  },
  actions: {
  },
  modules: {
  }
})
