<template>
  <div>
  <nav-bar @go-home="resetFormStep"/>
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
      },
      resetFormStep(){
        // if('booking_form' in this.$refs)
        //   this.$refs.booking_form.
      }
  },
  created: function () {
  this.$store.dispatch('fetchCustomerWhoami')
    .then(data => {
        let isAuthenticated = data.authenticated == true;
        if(!isAuthenticated){
          this.$bvModal.show('login-modal')
        }
    }).catch( (err) => {
        this.$bvModal.show('login-modal')
        console.log(err)
    });
  },
}

</script>

<style>
</style>
