<template>
    <div class="queue my-2">
    <b-card bg-variant="default" header="Info" class="text-center my-3">
    <b-overlay :show="busy">
        <div v-if="!expiredTicket">
            <p>
            {{queue.people}} people in queue
            </p>
            <p>
            Estimated waiting time:
            {{est}}
            </p>
        </div>
        <div v-if="expiredTicket">
            <p>
                This ticket has expired.
            </p>
        </div>
    </b-overlay>
    </b-card>
    <b-card bg-variant="primary" text-variant="white" header="Tip" class="my-2 text-center small">
        <b-card-text>Remember to take into account the time needed to reach the store.</b-card-text>
    </b-card>
    </div>
</template>
<script>
export default {
    props:{
        shop_id: String,
        ticket_id: String,
    },
    data(){
        return {
            queue: {},
            expiredTicket: false,
            busy: false,
        }
    },
    computed:{
        est() {
            let then = new Date(this.queue.est+"Z")
            let now = new Date()
            let secs = (then - now)/1000;
            let min = Math.round(secs/60,3)
            return min +"  minutes";
        }
    },
    watch:{
        shop_id(){
            this.fetchEstimatedTime()
        },
        ticket_id(){
            this.fetchEstimatedTime()
        }
    },
    methods: {
        fetchEstimatedTime(){
            if(!this.shop_id)
                return 
            this.busy = true;
            let query;
            if(this.ticket_id)
                query = () => { return this.$api.get("/ticket/est?uid="+ encodeURIComponent(this.ticket_id)) }
            else
                query = () => { return this.$api.get("/shop/"+this.shop_id+"/ticket/queue")}

            query()
            .then(res => {
            if(res.status == '200'){
                this.queue = res.data
                //this.tickets = res.data.tickets;
            }
            }).catch( (err) => {
                if(err.response.status == 400){
                    if(err.response.data === "Expired or invalid ticket")
                        this.expiredTicket = true
                }else{
                    this.$emit('connection-failure')
                }
            }).finally( () => {
                setTimeout( () => {this.busy = false}, 250)
            })
        }
    },
    created(){
        this.fetchEstimatedTime()
    }
}
</script>