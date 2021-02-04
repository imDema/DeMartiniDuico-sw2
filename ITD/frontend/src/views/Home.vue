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
}

</script>

<style>
</style>
