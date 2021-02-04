<template>
  <b-navbar type="dark" toggleable="lg" variant="primary">
  <b-navbar-brand to="/">
    <img id="logo" :src="require('../assets/logo-CLup-mini.png')" class="d-inline-block" alt="logo"/>
    CLup
  </b-navbar-brand>
  <b-navbar-nav class="ml-auto mr-2">
    <b-nav-item to="/tokens">
     <b-icon icon="upc-scan"/> My tokens
    </b-nav-item>
  </b-navbar-nav>
  <b-navbar-toggle target="nav-collapse">
      <template #default="{ expanded }">
        <b-icon v-if="expanded" icon="chevron-compact-up"></b-icon>
        <b-icon v-else icon="three-dots"></b-icon>
      </template>
  </b-navbar-toggle>
      <b-collapse id="nav-collapse" is-nav>
      <b-navbar-nav v-if="!$store.state.email" class="ml-auto">
        <b-nav-text class="text-light">Logged out</b-nav-text>
        <b-nav-item href="#" @click="displayLogin">
          <b-icon icon="lock"/>   
          Login
        </b-nav-item>
      </b-navbar-nav>
      <b-navbar-nav v-if="$store.state.email" class="ml-auto">
        <!--<b-nav-item href="#" @click="displayLogin">Display login</b-nav-item>-->
        <b-nav-text class="text-light">Logged in as <span class="bold">{{$store.state.email}}</span></b-nav-text>
        <b-nav-item href="#" @click="logout">   
          <b-icon icon="power" aria-hidden="true"></b-icon> 
          Logout
          </b-nav-item>
      </b-navbar-nav>
      </b-collapse>
  </b-navbar>
</template>

<script>
  export default {
    computed:{

    },
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
            this.$emit('logout')
            this.$router.replace('/')
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