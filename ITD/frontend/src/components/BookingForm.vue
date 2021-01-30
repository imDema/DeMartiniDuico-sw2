<template>
  <div class="container my-4">
  <header>
  <h2 class="my-2">{{store_info.name}}<b-button :href="maps_url" target="_blank" class="mx-2" variant="outline-secondary"><b-icon-map/>Open in Maps </b-button></h2>
  <span class="italic">{{store_info.description}}</span> 
  </header>
  <b-form class="py-2" novalidate>
      <b-form-group id="input-group-categories" label="Categories:">         
        <b-form-checkbox v-for="cat in store_info.departments" :key="cat.uid" :value="cat.uid" :checked="form.categories.indexOf(cat.uid)!==-1" v-model="form.categories">
          {{ cat.description }}
        </b-form-checkbox>
      </b-form-group>
    <b-form-group v-if="isBooking">
      <label for="example-datepicker">Choose a time:</label>
      <b-form-datepicker v-model="form.datetime.date" class="mb-2" required></b-form-datepicker>
      <b-form-timepicker v-model="form.datetime.time" class="mb-2" required></b-form-timepicker>
    </b-form-group>
  <queue :shop_id="store.id"/>
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
        store_info: {},
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
      }
    },
    computed: {
      categoriesValidation(){
        return true
      },
      maps_url(){
        if(Object.keys(this.store_info).length === 0)
          return "#"
        let coord = this.store_info.location;
        let matches = /^(\d+\.\d+)([NS]),(\d+\.\d+)+([EW])$/g.exec(coord)
        if(!matches)
          return "#"
        let lat  = ((matches[2]==='N')?'':'-') + matches[1]
        let long = ((matches[4]==='E')?'':'-') + matches[3]
        console.log(lat)
        return "https://bing.com/maps/default.aspx?rtp=~pos."+lat+"_"+long
      }
    },
    // created() {
    //   this.fetchStoreInfo(){
    // },
    watch:{
      store(newStore){
        newStore
        this.fetchStoreInfo()
      }
    },
    created(){
      this.fetchStoreInfo()
    },
    methods: {
      prev() {
      this.step--;
      },
      next() {
      this.step++;
      },
      fetchStoreInfo(){
        if(!this.store.id)
          return
        this.$api.get("/shop/"+this.store.id)
        .then((res)=>{
          console.log(res.data)
          this.store_info = res.data
          // //BEGIN temp
          // if(this.store.id === "dc73e9ce"){
          //   this.categories = [
          //     {id: "f2804cb5", value: "Frutta"},
          //     {id: "4b728f24", value: "Pane"}
          //   ]
          // }else if(this.store.id === "f02465ad"){
          //   this.categories = [
          //       {id: "643014bb", value: "Surgelati"},
          //       {id: "8a31d9d8", value: "Carne"},
          //       {id: "9c1bbbf3", value: "Pane"}
          //   ]
          // }else if(this.store.id === "a6692a21"){
          //   this.categories = [
          //     {id: "3c4a7133", value: "all"}
          //   ]
          // }
          // //END temp
        })
        .catch( (err) => {
          console.log(err)
        } );
      },
      submitTicket(e) {
        e.preventDefault()
        if(!this.store.id){
          alert("Store data missing");
          return
        }
        let endpoint = "/shop/"+this.store.id+"/ticket/new"
        let est_minutes = 30
        this.$api.post(endpoint, {
          est_minutes: est_minutes,
          department_ids: this.form.categories
        })
        .then(res => {
          if(res.status == '200'){
            this.$emit('success')
            this.$router.push('/tokens/d5075936')
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