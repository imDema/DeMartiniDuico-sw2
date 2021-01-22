<template>
    <!--<BookingForm v-if="step === 1" :isBooking="isBooking" @submit="next"/>-->

  <div class="container">
 <h2>Unes Milano</h2>
<div class="py-2">
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
      <b-button @click="submitTicket" type="submit" variant="primary" block>Submit</b-button>
  </div>
  </div>
</template>

<script>
  export default {
    data() {
      return {
        form: {
          step: 0,
          lastStep: 3,
          categories: [],
          city: {stores: {}},
          store: null,
          datetime: { date: null, time: null}
        },
        cities: [
          { text: 'Select One', value: null, disabled: true}, 
          {text: 'Milan', value: {name: 'Milan', stores: [{ text: 'Select a store', value: null, disabled: true},'unes', 'pippo', 'ciccio']}},
          {text: 'Rome', value: {name: 'Rome', stores: [{ text: 'Select a store', value: null, disabled: true}, 'unes', 'romeo shop', 'carrefurto']}},
          {text: 'Turin', value: {name: 'Turin', stores: [{ text: 'Select a store', value: null, disabled: true}, 'unes', 'scotti market', 'esselunga']}},
        ],
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
      }
    },
    methods: {
      prev() {
      this.step--;
      },
      next() {
      this.step++;
      },
      submitForm(evt) {
      evt.preventDefault();
          if(this.step==this.lastStep){
              alert('Submit to blah and show blah and etc.');
          }
      },
      submitTicket() {
        if(!this.form.city || !this.form.store)
          alert("Data missing");
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