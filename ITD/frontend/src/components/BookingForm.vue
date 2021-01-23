<template>
  <div class="container my-4">
  <h2 class="my-2">{{store.value}}<b-button :href="store.maps_url" target="_blank" class="mx-2" variant="outline-secondary"><b-icon-map/>Open in Maps </b-button></h2>
  <b-form class="py-2" novalidate>
      <b-form-group id="input-group-3" label="Categories:" label-for="input-3">         
        <b-form-checkbox v-for="cat in categories" :key="cat.value" :value="cat.value" :checked="form.categories.indexOf(cat.value)!==-1" v-model="form.categories">
          {{ cat.text }}
        </b-form-checkbox>
      </b-form-group>
    <b-form-group v-if="isBooking">
      <label for="example-datepicker">Choose a time:</label>
      <b-form-datepicker v-model="form.datetime.date" class="mb-2" required></b-form-datepicker>
      <b-form-timepicker v-model="form.datetime.time" class="mb-2" required></b-form-timepicker>
    </b-form-group>
    <b-row class="my-4">
    <b-col cols="6"><b-button @click="$emit('back')" block><b-icon-arrow-left/> Back</b-button> </b-col>
    <b-col cols="6"><b-button @click="submitTicket" type="submit" variant="primary" block>Submit</b-button></b-col>
    </b-row>
    <b-tooltip target="booking-button" triggers="hover" placement="bottom">Booking is not available in this demo</b-tooltip>
  </b-form>
  </div>
</template>

<script>
  export default {
    data() {
      return {
        form: {
          step: 0,
          categories: [],
          datetime: { date: null, time: null}
        },
        categories: [
          {value: 1, text: 'Meat and Fish', selected: false}, 
          {value: 2, text: 'Bread', selected: false}, 
          {value: 3, text: 'Dairy products', selected: false}, 
          {value: 4, text: 'Canned food', selected: false}, 
          {value: 5, text: 'Cereals, Pasta etc.', selected: false}, 
          {value: 6, text: 'Beverages', selected: false}, 
          {value: 7, text: 'Frozen food', selected: false}, 
          {value: 8, text: 'Household items', selected: false}, 
          ],
        show: true
      }
    },
    props: {
      isBooking: { 
        type: Boolean,
        default: false,
      },
      store: {
        type: Object,
      }
    },
    computed: {
      categoriesValidation(){
        return true
      }
    },
    methods: {
      prev() {
      this.step--;
      },
      next() {
      this.step++;
      },
      submitTicket() {
        if(!this.store.id){
          alert("Store data missing");
          return
        }
        let endpoint = "/shop/"+this.store.id+"/ticket/new"
        this.$api.post(endpoint, {
          department_ids: this.form.categories
        })
        .then(res => {
          if(res.status == '200'){
            this.$emit('success')
          }
        }).catch( () => {})
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