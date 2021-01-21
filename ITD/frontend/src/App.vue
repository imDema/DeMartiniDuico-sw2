<template>
<router-view/>
</template>

<script>
export default {
  created: function () {
  // axios.interceptors.response.use(undefined, function (err) {
  //   return new Promise(function (resolve, reject) {
  //     if (err.status === 401 && err.config && !err.config.__isRetryRequest) {
  //     // if you ever get an unauthorized, logout the user
  //     //open login modal
  //     }
  //     throw err;
  //   });
  // });
  
  //IF auth fails
  //set logout in vuex

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
