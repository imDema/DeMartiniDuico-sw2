<template>
<div>
    <div class="mt-3 logo-container">
    <b-img class="logo-big" alt="CLup logo" :src="require('../assets/logo-CLup_1.png')" center fluid></b-img>
    </div>
    <div class="container px-4">
    <p v-if="errors.length">
    <b>Please correct the following error(s):</b>
    <ul>
      <li v-for="error in errors" :key="error">{{ error }}</li>
    </ul>
    </p>
    <SearchBar @submit="submitSearch"/>
    <b-row class="my-5">
    <b-col cols="6"><b-button class="h-100" variant="success" id="ticket-button" @click="selectTicket" block>Get a Ticket</b-button></b-col>
    <b-col cols="6"><b-button class="h-100" variant="primary" id="booking-button" block>Make a Booking</b-button>
    <b-tooltip target="booking-button" triggers="hover" placement="bottom">Booking is not available in this demo</b-tooltip>
    </b-col>
    </b-row>
    </div>
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
            searchChoice: {},
        }
    },
    components: {
        SearchBar,
    },
    methods: {
        submitSearch(choice){
            this.searchChoice = choice;
        },
        selectTicket() {
            this.isBooking = false;
            if(!this.searchChoice){
                alert('pick a result')
            }
            this.$emit('get-ticket', this.searchChoice)
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