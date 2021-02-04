<template>
  <div>
  <b-jumbotron class="py-1 px-3" bg-variant="secondary" text-variant="white" border-variant="dark">
    Your tokens
  </b-jumbotron>
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
    <div class="d-flex w-100 justify-content-between">
      <div>
      <p class="mb-1">
        {{ shopDescription[t.shop_id] }}
      </p>
      <small>Departments: {{ departmentNames[t.shop_id] }}</small>
      </div>
      <div>
        <b-button  :id="'popover-delete-event' + t.uid" @click="deleteOnClick"><b-icon icon="trash"/></b-button>
            <b-popover :target="'popover-delete-event'+ t.uid" triggers="focus" placement="left">
              <div>Are you sure you want to delete this ticket?</div>
              <b-button variant="danger" @click="deleteTicket(t.uid)">Delete</b-button>
            </b-popover>
      </div>
    </div>
  </b-list-group-item>
  </b-list-group>
  <token-display v-if="isTicketSelected" :ticket="selectedTicket" :shop-description="shopDescription[selectedTicket.shop_id]" :departments="departmentNames[selectedTicket.shop_id]"/>
  <b-row class="my-4">
    <b-col cols="6"><b-button @click="back" block><b-icon-arrow-left/>Back</b-button> </b-col>
    <b-col cols="6" v-if="isTicketSelected"><b-button @click="showQR" variant="primary" block>Show</b-button></b-col>
  </b-row>

  </div>
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
          shopInfo: {},
          departmentNames: {},
          shopDescription: {},
      }
  },
  computed:{
    isTicketSelected(){
      return Object.keys(this.selectedTicket).length !== 0
    },        
  },
  watch:{
    $route(to){
      this.onRouteChange(to)
    },
    async tickets(newTickets){
      this.loadShopInfo(newTickets);
    }
  },
  methods: {
      onRouteChange(to){
        if(!to.params.uid){
          //console.log('empty uid')
          this.$set(this, 'selectedTicket', {})
        }
        this.loadTokens().then( (newTickets) => {
          if(to.params.uid){
            let selectedTickedUID = to.params.uid
            this.selectTicket(this.tickets.find( t => t.uid === selectedTickedUID))
          }
          this.loadShopInfo(newTickets)
        });
      },
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
          .catch( (err) => {
              if(err.name === 'NavigationDuplicated'){
                //ignore
              }else{
                console.log(err)
              }
            })
          this.selectedTicket = ticket
      },
      loadTokens(){
        this.busy = true;
        return this.$api.get("/tokens", { })
        .then(res => {
          if(res.status == '200'){
            this.tickets = res.data.tickets;
            return this.tickets
          }
        }).catch( () => {
            this.$emit('connection-failure')
        }).finally( () => {
          setTimeout( () => {this.busy = false}, 250)
        })
      },
      loadShopInfo(tickets){
          let fetchPromises = []
          tickets.map(t => t.shop_id).filter((v, i, a) => a.indexOf(v) === i)
          .forEach( (sid) => {
            fetchPromises.push(
              this.$api.get("/shop/"+sid)
              .then((res)=>{
                this.shopInfo[sid] = res.data;
              })
              .catch( (err) => {
                console.log(err)
              })   
            )
          });
          Promise.all(fetchPromises).then( () => {
            this.updateShopDescription()
            this.updateDepartmentNames()
          });
      },
      updateShopDescription(){
        this.tickets.forEach( (t) => {
            if(t.shop_id  in this.shopInfo)
              this.$set(this.shopDescription, t.shop_id, this.shopInfo[t.shop_id].description)
            else
              this.$set(this.shopDescription, t.shop_id, "")
        })
      },
      updateDepartmentNames(){
        this.tickets.forEach( (t) => {
            if(t.shop_id  in this.shopInfo)
              this.$set(this.departmentNames, t.shop_id, this.shopInfo[t.shop_id].departments
                .filter(d => t.department_ids.indexOf(d.uid)!==-1).map(d => d.description).join(", ")
              )
            else
              this.$set(this.departmentNames, t.shop_id, "")
        })
      },
      deleteOnClick(e){
          e.preventDefault()
          e.stopPropagation()
      },
      deleteTicket(uid){
          this.$api.post("/ticket/cancel", {uid: uid})
          .then(this.loadTokens)
          .catch(console.log)
      },
      
  },
  created(){
    this.onRouteChange(this.$route)
  },
}

function prettyDateDiff(millisecs) {
  var secs = Math.floor(Math.abs(millisecs) / 1000);
  if (secs < 60) return secs + " sec(s)";
  if (secs < 3600) return Math.floor(secs / 60) + " min(s)";
  if (secs < 86400) return Math.floor(secs / 3600) + " hour(s)";
  if (secs < 604800) return Math.floor(secs / 86400) + " day(s)";
  return "Months ";
}
</script>

<style>
</style>
