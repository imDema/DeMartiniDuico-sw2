<template>
  <b-navbar type="dark" toggleable="lg" variant="primary">
  <b-navbar-brand href="#">
    <img id="logo" :src="require('../assets/logo-CLup-mini.png')" class="d-inline-block" alt="logo"/>
    CLup
  </b-navbar-brand>
  <b-navbar-nav class="ml-auto mr-2">
    <b-nav-item>My tokens</b-nav-item>
  </b-navbar-nav>
  <b-navbar-toggle target="nav-collapse">
      <template #default="{ expanded }">
        <b-icon v-if="expanded" icon="chevron-compact-up"></b-icon>
        <b-icon v-else icon="three-dots"></b-icon>
      </template>
  </b-navbar-toggle>
      <b-collapse id="nav-collapse" is-nav>
      <b-navbar-nav class="ml-auto">
        <!--<b-nav-item href="#" @click="displayLogin">Display login</b-nav-item>-->
        <b-nav-item href="#" @click="logout">Logout</b-nav-item>
      </b-navbar-nav>
      </b-collapse>
  </b-navbar>
</template>

<script>
  export default {
    methods: {
      displayLogin(){
        this.$bvModal.show('login-modal')
      },
      logout(){
        //OPTIONAL 
        //show modal:
        //Are you sure you want to logout?
        this.$api.get("/logout")
        .then( res => {
          if(res.status == '200'){
            this.$store.commit('logged_out')
            console.log('successful logout')
            this.displayLogin()
          }
        })
        .catch( err => {
          console.log(err)
          console.log('failed logout')
        })
      }
    },
  }
</script>

<style scoped>
  #logo {
    width: 40px;
    height: 40px;
  }
</style>