<template>
    <b-modal id="scan-modal" hide-footer busy>
    <template #modal-header>
        <div class="modal_header">Scan token</div>
    </template>
    <b-form @submit.stop.prevent="submit">
        <b-form-group
        id="input-group-staff-1"
        label="Token code:"
        label-for="input-1"
        description="Insert the 8 digit UID of the token">
            <b-form-input
            id="input-staff-1"
            v-model="form.uid"  
            required
            placeholder="Enter code"
            :state="validation"
            ></b-form-input>
      </b-form-group>
        <b-overlay       
        :show="isSubmitBusy"
        rounded
        opacity="0.3"
        spinner-small
        spinner-variant="primary"
        >
        <b-button type="submit" variant="primary" class="btn-block">Submit</b-button>
      </b-overlay>
    </b-form>
  </b-modal>
</template>
<script>
export default {
    components:{
    },
    data() { 
        return {
            form: {
                uid: ''
            },   
            isSubmitBusy: false,
        }
    },
    props:{
    },
    computed:{
        validation() {
            var uid = this.form.uid
            return uid.length >= 4
        },
    },
    methods: {
        submit(){
            console.log('submit')
            this.isSubmitBusy = true

            let uid = this.form.uid;
            if(!this.validation)
                return //TODO error message
            this.$router.push('/staff/tokens/'+uid)
        },
    }
}
</script>
<style scoped>
.modal_header{
    font-size: 1.5rem;
}
</style>