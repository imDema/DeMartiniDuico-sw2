<template>
    <div>
    <h3 class="my-2">{{ticket.shop_id}}<b-button :href="maps_url" target="_blank" class="mx-2" variant="outline-secondary"><b-icon-map/>Open in Maps </b-button></h3>
    Creation: {{creation}}
    Expiration: {{expiration}}
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
            let d = new Date(this.ticket.creation+"Z");
            return d.toLocaleDateString() + " at " + d.toLocaleTimeString();

        },
        expiration(){
            let d = new Date(this.ticket.expiration+"Z");
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