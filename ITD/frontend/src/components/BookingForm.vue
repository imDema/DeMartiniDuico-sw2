<template>
  <div class="container my-4">
  <header>
  <h2 class="my-2">{{store.name}}<b-button :href="maps_url" target="_blank" class="mx-2" variant="outline-secondary"><b-icon-map/>Open in Maps </b-button></h2>
  <span class="italic">{{store.description}}</span> 
  </header>
  <b-form class="py-2" novalidate>
      <b-form-group id="input-group-categories" label="Departments:" v-if="'departments' in store && store.departments.length >=2">         
        <b-form-checkbox v-for="cat in store.departments" :key="cat.uid" :value="cat.uid" :checked="form.categories.indexOf(cat.uid)!==-1" v-model="form.categories">
          {{ cat.description }}
        </b-form-checkbox>
      </b-form-group>
    <b-form-group v-if="isBooking">
      <label for="example-datepicker">Choose a time:</label>
      <b-form-datepicker v-model="form.datetime.date" class="mb-2" required></b-form-datepicker>
      <b-form-timepicker v-model="form.datetime.time" class="mb-2" required></b-form-timepicker>
    </b-form-group>
  <queue :shop_id="store.uid"/>
    <b-row class="my-4">
    <b-col cols="6"><b-button @click="$emit('back')" block><b-icon-arrow-left/> Back</b-button> </b-col>
    <b-col cols="6"><b-button @click="submitTicket" type="submit" variant="primary" block>Submit</b-button></b-col>
    </b-row>
    <b-tooltip target="booking-button" triggers="hover" placement="bottom">Booking is not available in this demo</b-tooltip>
  </b-form>
  </div>
</template>

<script>
import Queue from "./Queue"
  export default {
    components:{
      Queue
    },
    data() {
      return {
        form: {
          step: 0,
          categories: [],
          datetime: { date: null, time: null}
        },
        //store_info: {},
        show: true
      }
    },
    props: {
      isBooking: { 
        type: Boolean,
        default: false,
      },
      // TODO add specific keys
      store: {
        type: Object,
        default() { return { departments: [] }},
      }
    },
    computed: {
      categoriesValidation(){
        return true
      },
      maps_url(){
        if(Object.keys(this.store).length === 0)
          return "#"
        let coord = this.store.location;
        let matches = /^(\d+\.\d+)([NS]),(\d+\.\d+)+([EW])$/g.exec(coord)
        if(!matches)
          return "#"
        let lat  = ((matches[2]==='N')?'':'-') + matches[1]
        let long = ((matches[4]==='E')?'':'-') + matches[3]
        return "https://bing.com/maps/default.aspx?rtp=~pos."+lat+"_"+long
      }
    },
    watch:{
      //store(newStore){
        //newStore
        //this.fetchStoreInfo()
      //}
    },
    created(){
      //this.fetchStoreInfo()
    },
    methods: {
      prev() {
      this.step--;
      },
      next() {
      this.step++;
      },
      //UNUSED
      // fetchStoreInfo(){
      //   if(!this.store.uid)
      //     return
      //   this.$api.get("/shop/"+this.store.uid)
      //   .then((res)=>{
      //     console.log(res.data)
      //     this.store_info = res.data
      //   })
      //   .catch( (err) => {
      //     console.log(err)
      //   } );
      // },
      submitTicket(e) {
        e.preventDefault()
        if(!this.store.uid){
          alert("Store data missing");
          return
        }
        if(!this.form.categories || this.form.categories.length === 0){
          this.form.categories = this.store.departments.map( d => d.uid);
        }
        let endpoint = "/shop/"+this.store.uid+"/ticket/new"
        let est_minutes = 30
        this.$api.post(endpoint, {
          est_minutes: est_minutes,
          department_ids: this.form.categories
        })
        .then(res => {
          if(res.status == '200'){
            this.$emit('success')
            let new_uid = res.data.uid
            this.$router.push('/tokens/'+new_uid)
            .catch( (err) => {
              if(err.name === 'NavigationDuplicated'){
                //ignore
              }else{
                console.log(err)
              }
            })
          }
        }).catch( () => {}) //TODO
      },
      onSubmit(evt){
        evt.preventDefault()
        this.submitBooking();
      },
      submitBooking() {
        alert(JSON.stringify(this.form))
      },
      // onReset(evt) {
      //   evt.preventDefault()
      //   // Reset our form values
      //   this.form.name = ''
      //   this.form.city = null
      //   this.form.store = null
      //   this.form.categories = []
      //   this.form.datetime.date = null;
      //   this.form.datetime.time = null;
      //   // Trick to reset/clear native browser form validation state
      //   this.show = false
      //   this.$nextTick(() => {
      //     this.show = true
      //   })
      // }
    }
  }
</script>