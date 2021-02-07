<template>
<div class="container my-4">
    <h4>Occupancy status (per department)</h4>
    <div class="my-2">
        <div v-if="!loaded">
            Loading occupancy data...
        </div>
        <div v-if="loaded && (!status || status.length===0)">
            No customers are currently visiting the store.
        </div>
        <b-list-group v-if="loaded && status.length">
        <b-list-group-item v-for="elem in status" :key="elem.department.uid">
              <div class="d-flex w-100 justify-content-left">
                <div>
                {{elem.department.description}}
                </div>
                <div class="mx-2">
                <b-badge :variant="(elem.occupancy/elem.department.capacity>0.5)?'danger':(elem.occupancy>0?'primary':'secondary')" pill>
                    {{elem.occupancy}}/{{elem.department.capacity}}
                </b-badge>
                </div>
            </div>
        </b-list-group-item>
        </b-list-group>
    </div>
    <b-row>
        <b-col cols="6"><b-button class="h-100" to="/staff" block><b-icon-arrow-left/> Back</b-button>
        </b-col>
    </b-row>
           <b-alert
          :show="successfulActionAlert.countdown"
          dismissible
          fade
          class="position-fixed fixed-bottom m-0 rounded-0"
          style="z-index: 2000;"
          variant="success"
          @dismiss-count-down="successfulActionAlert.countdown=$event"
        >
          {{successfulActionAlert.message}}
        </b-alert>
        <b-alert
          :show="failedActionAlert.countdown"
          dismissible
          fade
          class="position-fixed fixed-bottom m-0 rounded-0"
          style="z-index: 2000;"
          variant="danger"
          @dismiss-count-down="failedActionAlert.countdown=$event"
        >
          {{failedActionAlert.message}}
        </b-alert>
</div>
</template>
<script>
export default {
    props:{
    },
    data(){
        return {
            status: [],
            loaded: false,
            successfulActionAlert:{
                countdown: 0,
                message: "Successful action",
            },
            failedActionAlert:{
                countdown: 0,
                message: "Failed action",
            }
        }
    },
    watch:{
        $route(to){
            this.onRouteChange(to)
        },
    },
    methods:{
        fetchOccupancy(shop_id){
            if(!shop_id)
              return
            return this.$api.get(`/staff/shop/${shop_id}/status`)
                .then(res => {
                    if(res.status == '200'){
                        this.status = res.data;
                        return this.status
                    }
                }).catch( console.log )
        },
        loadInfo(){
            this.loaded = false
            let whoami = this.$store.dispatch('fetchStaffWhoami')
            whoami.then( data => this.fetchOccupancy(data.shop_id))
            .catch(this.showFailedActionAlert)
            .finally( () => {
                this.loaded = true
            });
        },
        logEntry(){
            this.logAction('log-entry')
        },
        logExit(){
            this.logAction('log-exit')
        },
        logAction(endpoint){
            if(!(endpoint == 'log-entry' || endpoint == 'log-exit'))
                return
            let shop_id = this.$store.state.staff.shop_id
            if(!this.uid || !shop_id){
                alert("Missing data required to perform this action.")
                return
            }
            this.$api.post(`/staff/shop/${shop_id}/token/${endpoint}`,
                { uid: this.uid }
            ).then( (response) => {
                response;
                this.showSuccessfulActionAlert("Successfully executed action: "+endpoint)
            })
            .catch((err) => this.showFailedActionAlert("Operation failed"+(err.response.data?":\n":"")+err.response.data))
        },
        showSuccessfulActionAlert(message){
            this.successfulActionAlert.message = message
            this.successfulActionAlert.countdown = 3
        },
        showFailedActionAlert(message){
            this.failedActionAlert.message = message
            this.failedActionAlert.countdown = 3
        },
    },
    computed:{
    },
    created(){
        this.loadInfo()
    },
}
</script>