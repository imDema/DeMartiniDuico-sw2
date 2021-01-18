<template>
    <div class="container px-4">
    <p v-if="errors.length">
    <b>Please correct the following error(s):</b>
    <ul>
      <li v-for="error in errors" :key="error">{{ error }}</li>
    </ul>
    </p>
    <SearchBar @submit="submitSearch"/>
    <b-row class="my-5">
    <b-col col="6"><b-button class="h-100" variant="secondary" @click="selectTicket" block>Get a Ticket</b-button></b-col>
    <b-col col="6"><b-button class="h-100" variant="primary" @click="selectBooking" block>Make a Booking</b-button></b-col>
    </b-row>
    </div>
</template>

<script>
import SearchBar from './SearchBar'
//import BookingForm from './BookingForm'
export default {
    data() { 
        return {
        errors: [],
        isBooking: false,
        }
    },
    components: {
        SearchBar,
    },
    methods: {
        submitSearch(){},
        selectTicket() {
            this.isBooking = false;
            //if(this.step===0)
            // this.next();
        },
        selectBooking() {
            this.isBooking = true;
            //if(this.step===0)
            //    this.next();
        }
    }
}
</script>

<style>

</style>