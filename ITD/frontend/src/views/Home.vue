<template>
  <div>
  <NavBar/>
  <div class="mt-3 logo-container">
  <b-img class="logo-big" alt="CLup logo" :src="require('../assets/logo-CLup_1.png')" center fluid></b-img>
  </div>
  <AuthModal/>
  <SearchForm/>
  </div>
</template>

<script>
import NavBar from '../components/NavBar'
import SearchForm from '../components/SearchForm'
import AuthModal from '../components/AuthModal'

export default {
  name: 'App',
  components: {
    NavBar,
    SearchForm,
    AuthModal
  },
  methods: {
  },
  created(){
          this.$api.get("/checkauth")
        .then(res => {
            let isAuthenticated = res.data == true;
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
  }
}
</script>

<style>
.logo-container{
  max-height: 200px;
  overflow: hidden;
}

</style>
