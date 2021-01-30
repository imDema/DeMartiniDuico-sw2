<template>
    <div class="queue my-2">
        <p>
        {{queue.people}} people in queue
        Estimated waiting time:
        {{queue.est}}
        </p>
        <div class="mt-2">
            <small>Remember to take into account the time needed to reach the store.</small>
        </div>
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
            queue: {}
        }
    },
    methods: {
        fetchEstimatedTime(){
            if(!this.shop_id)
                return 
            this.busy = true;
            let query;
            if(!this.ticket_id)
                query = () => { return this.$api.post("/ticket/est", { uid: this.ticket_id})}
            else
                query = () => { return this.$api.get("/shop/"+this.shop_id+"/ticket/queue")}

            query()
            .then(res => {
            if(res.status == '200'){
                this.queue = res.data
                //this.tickets = res.data.tickets;
            }
            }).catch( () => {
                this.$emit('connection-failure')
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