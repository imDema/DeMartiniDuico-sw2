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
      state.staff.shop_id = ""

    }   
  },
  actions: {
    fetchStaffWhoami (store) {
      return this._vm.$api.get("/staff/whoami")
      .then(res => {
          let isAuthenticated = res.data.authenticated == true;
          if(isAuthenticated){
            store.state.staff.shop_id = res.data.shop_id;
            store.state.staff.email = res.data.email;
            store.commit('staff_logged_in')
            console.log('Staff authenticated')
          }else{
            store.commit('staff_logged_out')
          }
          return res.data
      }).catch( (err) => {
          store.commit('staff_logged_out')
          console.log(err)
      });
    },
    fetchCustomerWhoami(store) {
      return this._vm.$api.get("/whoami")
      .then(res => {
          let isAuthenticated = res.data.authenticated == true;
          if(isAuthenticated){
            store.state.customer.email = res.data.email;
            store.commit('logged_in')
            console.log('User authenticated')
          }else{
            store.commit('logged_out')
          }
          return res.data
      }).catch( (err) => {
          store.commit('logged_out')
          console.log(err)
      });
    },
  },
  modules: {
  }
})
