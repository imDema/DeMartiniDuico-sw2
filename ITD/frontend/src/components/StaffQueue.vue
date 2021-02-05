<template>
<div class="my-3">
  <h4>Ticket queue</h4>
  <b-overlay :show="busy">
  <div v-if="!tickets || tickets.length === 0">
    No tickets in queue.
  </div>
  <b-list-group>
  <b-list-group-item  v-for="(t, index) in tickets" :key="t.uid" href="#" :active="t==selectedTicket" class="flex-column align-items-start" 
    @click="onTicketClick(t)"
  >
    <div class="d-flex w-100 justify-content-between">
      <div>
      <span>{{index+1}}) </span>
      <span class="h5 mb-1">{{t.uid}}</span>
      </div>
      <small>Created: {{timeDifference(t.creation)}}</small>
    </div>
    <div class="d-flex w-100 justify-content-between">
      <div>
      <p class="mb-1">
          Expiration: {{prettyDate(t.expiration)}}
      </p>
      <small>Department_ids: {{ t.department_ids }}</small>
      </div>
      <div>
        <b-button :id="'popover-delete-event' + t.uid" @click="skipOnClick"><b-icon icon="x-square"/></b-button>
            <b-popover :target="'popover-delete-event'+ t.uid" triggers="focus" placement="left">
              <div>Are you sure you want to skip this ticket?</div>
              <b-button variant="danger" @click="skipTicket(t.uid)">Skip</b-button>
            </b-popover>
      </div>
    </div>
  </b-list-group-item>
  </b-list-group>
  </b-overlay>
</div>
</template>
<script>
export default {
  data(){
    return {
      tickets:[],
      busy: false,
      selectedTicket: {},
      loadTokensInterval: {},
    }
  },
  methods:{
    onTicketClick(t){
      if(!t.uid)
        return
      this.$router.push('/staff/tokens/'+t.uid)
    },
    skipOnClick(e){
      e.preventDefault()
      e.stopPropagation()
    },
    skipTicket(uid){
        let shop_id = this.$store.state.staff.shop_id
        this.$api.post(`/staff/shop/${shop_id}/token/skip`, {uid: uid})
        .then(this.loadTokens)
        .catch(console.log)
    },
    timeDifference(time){
        var now = Date.now();
        var then = new Date(time).getTime();
        
        let diff = then - now;
        let diffText = prettyDateDiff(diff)
        return diff>0?("In "+ diffText):(diffText+" ago")
    },
    loadTokens(){
      this.busy = true;
      return new Promise((resolve, reject) => {
        let shop_id = this.$store.state.staff.shop_id
        if(!shop_id){
          return this.$store.dispatch('fetchStaffWhoami').then(resolve).catch(reject)
        }else{
          return resolve({shop_id: shop_id})
        }
      })
      .then( (data) => {
        data;
        let shop_id = data.shop_id
        if(!shop_id)
          return
        return this.$api.get(`/staff/shop/${shop_id}/ticket/queue`)
        .then(res => {
          if(res.status == '200'){
            this.tickets = res.data;
            return this.tickets
          }
        }).catch( () => {
            this.$emit('connection-failure')
        })
      }).finally( () => {
          setTimeout( () => {this.busy = false}, 250)
      })
    },
    // loadShopInfo(){
    //                 this.$api.get("/shop/"+sid)
    //           .then((res)=>{
    //             this.shopInfo[sid] = res.data;
    //           })
    //           .catch( (err) => {
    //             console.log(err)
    //           })   
    //         )
    // }
    prettyDate(date){
      let d = new Date(date)
      return d.toLocaleString()
    },
  },
  created(){
    // this.loadShopInfo()
    // .then(this.loadTokens);
    setTimeout(this.loadTokens, 100)
    setTimeout(() => {
      this.loadTokensInterval = setInterval(this.loadTokens, 4000)
    },
    1000)
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