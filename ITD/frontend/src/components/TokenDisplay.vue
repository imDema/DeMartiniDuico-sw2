<template>
    <div>
    <h3 class="my-2">{{ticket.shop_name}}<b-button :href="maps_url" target="_blank" class="mx-2" variant="outline-secondary"><b-icon-map/>Open in Maps </b-button></h3>
    <div class="my-2">
        {{shopDescription}}
    </div>
    <b-card bg-variant="white">   
        <div>
        Creation: {{creation}}
        </div>
        <div>
        Expiration: {{expiration}}
        </div>
    </b-card>
    <b-card bg-variant="light">   
        Departments for this ticket: {{departments}}
    </b-card>
    <queue :shop_id="ticket.shop_id" :ticket_id="ticket.uid" />
    </div>
</template>
<script>
import Queue from "./Queue"
export default {
    components:{
        Queue
    },
    props: {
        ticket: Object,
        shopDescription: String,
        departments: String,
    },
    data(){
        return {
            busy: false,
            queue: {}
        }
    },
    computed:{
        maps_url(){
            //TODO
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
}
</script>
<style scoped>
    .queue {
        text-align: center;
    }
</style>