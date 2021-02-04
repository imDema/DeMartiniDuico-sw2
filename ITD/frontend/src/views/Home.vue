<template>
  <div>
  <nav-bar/>
  <auth-modal @connection-failure="showFailedToConnectAlert"/>
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
  this.$api.get("/whoami")
    .then(res => {
        let isAuthenticated = res.data.authenticated == true;
        this.$store.state.customer.email = res.data.email;
        if(isAuthenticated){
          console.log('true')
          this.$store.commit('logged_in')
        }else{
          this.$store.commit('logged_out')
          this.$bvModal.show('login-modal')
        }
    }).catch( (err) => {
        this.$bvModal.show('login-modal')
        console.log(err)
        console.log('error with /checkauth')
    });
  },
}

</script>

<style>
</style>
