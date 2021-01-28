<template>
  <div class="container my-4">
  <div v-if="tickets.length===0">
    <h3>No tokens available.</h3>
  </div>
  <b-list-group v-if="!isTicketSelected">
  <b-list-group-item  v-for="t in tickets" :key="t.id" href="#" :active="t==selectedTicket" class="flex-column align-items-start" 
    @click="onTicketClick(t)"
  >
    <div class="d-flex w-100 justify-content-between">
      <h5 class="mb-1">Shop name</h5>
      <small>{{timeDifference(t.creation)}}</small>
    </div>
    <p class="mb-1">
      Department names
    </p>

    <small>Details</small>
  </b-list-group-item>
  </b-list-group>
  <token-display v-if="isTicketSelected" :ticket="selectedTicket"/>
  <b-row class="my-4">
    <b-col cols="6"><b-button @click="back" block><b-icon-arrow-left/>Back</b-button> </b-col>
    <b-col cols="6" v-if="isTicketSelected"><b-button @click="showQR" variant="primary" block>Show</b-button></b-col>
  </b-row>

  </div>
</template>

<script>
import TokenDisplay from '../components/TokenDisplay.vue'

export default {
  name: 'App',
  components: {
    TokenDisplay
  },
  data() {
      return {
          showingQR: false,
          busy: false,
          selectedTicket: {},
          tickets: [],
      }
  },
  computed:{
    isTicketSelected(){
      return Object.keys(this.selectedTicket).length !== 0
    }
  },
  methods: {
      timeDifference(time){
        var now = Date.now();
        var then = new Date(time).getTime();
        
        let diff = then - now;
        let diffText = prettyDateDiff(diff)
        return diff>0?("In "+ diffText):(diffText+" ago")
      },
      back(){
        if(this.isTicketSelected){
          this.deselectTicket()
        }else{
          this.$router.push("/")
        }
      },
      logout(){
        //reset data
      },
      showQR(){
        alert(this.selectedTicket.uid)
      },
      onTicketClick(ticket){
        if(this.selectedTicket === ticket){
          this.deselectTicket()
        }else{
          this.selectTicket(ticket)
        }
      },
      deselectTicket(){
        this.selectedTicket = {}
      },
      selectTicket(ticket){
          this.selectedTicket = ticket
      },
      loadTokens(){
        this.busy = true;
        this.$api.get("/tokens", { })
        .then(res => {
          if(res.status == '200'){
            this.tickets = res.data.tickets;
          }
        }).catch( () => {
            this.$emit('connection-failure')
        }).finally( () => {
          setTimeout( () => {this.busy = false}, 250)
        })
      }
  },
  async mounted(){
    this.loadTokens()
  }
}

function prettyDateDiff(millisecs) {
  var secs = Math.floor(Math.abs(millisecs) / 1000);
  if (secs < 60) return secs + " sec(s)";
  if (secs < 3600) return Math.floor(secs / 60) + " min(s)";
  if (secs < 86400) return Math.floor(secs / 3600) + " hour(s)";
  if (secs < 604800) return Math.floor(secs / 86400) + " day(s)";
  return Date().toDateString();
}
</script>

<style>
</style>
