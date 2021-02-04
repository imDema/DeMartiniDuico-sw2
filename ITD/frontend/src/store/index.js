import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
  state: {
    customer: {
      loggedIn: false,
      email: ""
    },
    staff:{
      loggedIn: false,
      email: "",
      shop_id: "",
    }
  },
  mutations: {
    logged_in (state){
      state.customer.loggedIn = true;
    },
    logged_out (state){
      state.customer.loggedIn = false;
      state.customer.email = ""
    },
    staff_logged_in (state){
      state.staff.loggedIn = true;
    },
    staff_logged_out (state){
      state.staff.loggedIn = false;
      state.staff.email = ""
    }
    
  },
  actions: {
  },
  modules: {
  }
})
