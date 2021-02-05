<template>
  <div>
  <nav-bar staff/>
  <auth-modal staff @connection-failure="showFailedToConnectAlert"/>
  <router-view/>
  <b-alert
    :show="failedToConnectAlertCountdown"
    dismissible
    fade
    class="position-fixed fixed-bottom m-0 rounded-0"
    style="z-index: 2000;"
    variant="danger"
    @dismiss-count-down="failedToConnectAlertCountdown=$event"
  >
  Failed to connect to CLup servers.
  </b-alert>
  </div>
</template>

<script>
import NavBar from '../components/NavBar.vue'
import AuthModal from '../components/AuthModal.vue'

export default {
  name: 'App',
  components: {
    NavBar,
    AuthModal,
  },
  data(){
    return {
      failedToConnectAlertCountdown: 0,
    }
  },
  methods: {
      showFailedToConnectAlert(){
        this.failedToConnectAlertCountdown = 3
      }
  },
  created: function () {
    this.$store.dispatch('fetchStaffWhoami')
    .then( (data) => {
        if(!data.authenticated)
          this.$bvModal.show('login-modal')
    }).catch( (err) => {
        this.$bvModal.show('login-modal')
        console.log(err)
    });
    // this.$api.get("/staff/whoami")
    //   .then(res => {
    //       let isAuthenticated = res.data.authenticated == true;
    //       if(isAuthenticated){
    //         this.$store.state.staff.shop_id = res.data.shop_id;
    //         this.$store.state.staff.email = res.data.email;
    //         this.$store.commit('staff_logged_in')
    //         console.log('Cookie -> authenticated')
    //       }else{
    //         this.$store.commit('staff_logged_out')
    //         this.$bvModal.show('login-modal')
    //       }
    //   }).catch( (err) => {
    //       this.$bvModal.show('login-modal')
    //       console.log(err)
    //       console.log('error with /checkauth')
    //   });
    }
}

</script>

<style>
</style>
