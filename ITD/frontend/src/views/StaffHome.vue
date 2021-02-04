<template>
<div>
<!--  <b-jumbotron class="py-1 px-3" bg-variant="secondary" text-variant="white" border-variant="dark">
    Staff services
  </b-jumbotron>
-->
  <div class="container my-4">
    <h4>Ticket functions</h4>
    <div>
      <b-row>
        <b-col cols="6"><b-button class="h-100" variant="primary" block @click="scanToken">Scan token</b-button></b-col>
        <b-col cols="6"><b-button class="h-100" id="substitute-button" variant="primary" block>Generate substitute ticket</b-button>
        <b-tooltip target="substitute-button" triggers="hover" placement="bottom">Substitute tickets are not available in this demo</b-tooltip>
        </b-col>
      </b-row>
    </div>
  </div>
  <scan-modal />
</div>
</template>

<script>
import ScanModal from '../components/ScanModal.vue'
export default {
  name: 'App',
  components: {
    ScanModal

  },
  data() {
      return defaultData()
  },
  methods: {
    scanToken(){
      this.$bvModal.show('scan-modal')
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
