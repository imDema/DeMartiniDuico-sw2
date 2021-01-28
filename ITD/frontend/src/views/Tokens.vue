<template>
  <div class="container my-4">
  <h2 class="my-2">{{store.value}}<b-button :href="store.maps_url" target="_blank" class="mx-2" variant="outline-secondary"><b-icon-map/>Open in Maps </b-button></h2>
    <b-row class="my-4">
    <b-col cols="6"><b-button to="/" block><b-icon-arrow-left/>Back</b-button> </b-col>
    <b-col cols="6"><b-button @click="showQR" variant="primary" block>Show</b-button></b-col>
    </b-row>
  </div>
</template>

<script>
export default {
  name: 'App',
  components: {
  },
  data() {
      return {
          showingQR: false,
          store: {
              value: "Negozio Pippo",
              maps_url: ""
          }
      }
  },
  methods: {
      logout(){
        //reset data
      },
      showQR(){

      }
  }
}
</script>

<style>
</style>
