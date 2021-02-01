<template>
  <div class="container my-4">
  <div v-if="tickets.length===0">
    <h3>No tokens available.</h3>
  </div>
  <b-list-group v-if="!isTicketSelected">
  <b-list-group-item  v-for="t in tickets" :key="t.uid" href="#" :active="t==selectedTicket" class="flex-column align-items-start" 
    @click="onTicketClick(t)"
  >
    <div class="d-flex w-100 justify-content-between">
      <h5 class="mb-1">{{t.shop_name}}</h5>
      <small>{{timeDifference(t.creation)}}</small>
    </div>
    <p class="mb-1">
      {{getDepartmentNames(t).map( d => d.description)}}
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
          departmentNames: {},
      }
  },
  computed:{
    isTicketSelected(){
      return Object.keys(this.selectedTicket).length !== 0
    }
  },
  watch:{
    $route(to, from){
      //update selectedTicket
      to, from
    },
    tickets(newTickets){
      newTickets.forEach( (t) => {
        this.$api.get("/shop/"+t.shop_id)
        .then((res)=>{
          console.log(res.data)
          this.departmentNames[t] = res.data.departments.filter( d => { return t.department_ids.indexOf(d.uid)!==-1} )
        })
        .catch( (err) => {
          console.log(err)
        });
      });
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
        this.$router.push("/tokens")
        this.selectedTicket = {}
      },
      selectTicket(ticket){
          if(!ticket){
            alert("Invalid ticket id")
            return
          }
          this.$router.push("/tokens/"+ticket.uid)
          this.selectedTicket = ticket
      },
      loadTokens(){
        this.busy = true;
        return this.$api.get("/tokens", { })
        .then(res => {
          if(res.status == '200'){
            this.tickets = res.data.tickets;
          }
        }).catch( () => {
            this.$emit('connection-failure')
        }).finally( () => {
          setTimeout( () => {this.busy = false}, 250)
        })
      },
      getDepartmentNames(t){
        if(t in this.departmentNames)
          return this.departmentNames[t]
        else
          return []
      },
  },
  created(){
    this.loadTokens().then( () => {
      if(this.$route.params.uid){
        let selectedTickedUID = this.$route.params.uid
        this.selectTicket(this.tickets.find( t => t.uid === selectedTickedUID))
      }
    });
  },
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
