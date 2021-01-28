<template>
  <div>
  <search-form v-show="step==0" @get-ticket="getTicket" ref="searchform"/>
  <booking-form v-show="step==1" :store="searchChoice" @back="back"/>
  </div>
</template>

<script>
import SearchForm from '../components/SearchForm.vue'
import BookingForm from '../components/BookingForm.vue'

export default {
  name: 'App',
  components: {
    SearchForm,
    BookingForm,
  },
  data() {
      return defaultData()
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
    },
    logout(){
      Object.assign(this.$data, defaultData());
      this.$refs.searchform.reset()
    }
  },
}

function defaultData(){
  return {
    step: 0,
    searchChoice: {},
  }
}
</script>

<style>
</style>
