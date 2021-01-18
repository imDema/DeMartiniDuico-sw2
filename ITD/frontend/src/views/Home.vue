<template>
  <div>
  <NavBar/>
  <div class="mt-3 logo-container">
  <b-img class="logo-big" alt="CLup logo" :src="require('../assets/logo-CLup_1.png')" center fluid></b-img>
  </div>
  <AuthModal/>
  <MainForm/>
  </div>
</template>

<script>
import NavBar from '../components/NavBar'
import MainForm from '../components/MainForm'
import AuthModal from '../components/AuthModal'

export default {
  name: 'App',
  components: {
    NavBar,
    MainForm,
    AuthModal
  },
  methods: {
  }
}
</script>

<style>
.logo-container{
  max-height: 200px;
  overflow: hidden;
}

</style>
