<template>
  <div>
  <nav-bar/>
  <auth-modal/>
  <search-form v-show="step==0" @get-ticket="getTicket"/>
  <booking-form v-show="step==1" :store="searchChoice" @back="back"/>
  </div>
</template>

<script>
import NavBar from '../components/NavBar.vue'
import SearchForm from '../components/SearchForm.vue'
import AuthModal from '../components/AuthModal.vue'
import BookingForm from '../components/BookingForm.vue'

export default {
  name: 'App',
  components: {
    NavBar,
    SearchForm,
    AuthModal,
    BookingForm,
  },
  data() {
    return {
      step: 0,
      searchChoice: {},
    }
  },
  methods: {
    getTicket(searchChoice){
      if(!searchChoice){
        console.log('get-ticket event without params')
        return
      }
      this.searchChoice = searchChoice
      this.step = 1
    },
    back(){
      this.resetSearch()
      this.step = 0
    },
    resetSearch(){
      this.searchChoice = {}
    }
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
