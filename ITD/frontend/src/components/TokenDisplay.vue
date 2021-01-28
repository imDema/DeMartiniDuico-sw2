<template>
    <div>
    <h3 class="my-2">{{ticket.shop_id}}<b-button :href="maps_url" target="_blank" class="mx-2" variant="outline-secondary"><b-icon-map/>Open in Maps </b-button></h3>
    Creation: {{creation}}
    Expiration: {{expiration}}
    <b-overlay
       :show="busy" rounded="sm"
    >
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
    </b-overlay>
    </div>
</template>
<script>
export default {
    props: {
        ticket: Object,
    },
    data(){
        return {
            busy: false,
            queue: {}
        }
    },
    computed:{
        maps_url(){
            return "openstreetmap.org"
        },
        creation(){
            let d = new Date(this.ticket.creation);
            return d.toLocaleDateString() + " at " + d.toLocaleTimeString();

        },
        expiration(){
            let d = new Date(this.ticket.expiration);
            return d.toLocaleDateString() + " at " + d.toLocaleTimeString();
        }
    },
    methods: {
        fetchEstimatedTime(){
            if(!('shop_id' in this.ticket))
                return
            
            this.busy = true;
            this.$api.get("/shop/"+this.ticket.shop_id+"/ticket/queue", { })
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
<style scoped>
    .queue {
        text-align: center;
    }
</style>