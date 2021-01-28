<template>
<router-view/>
</template>

<script>
export default {
  created: function () {
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
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
