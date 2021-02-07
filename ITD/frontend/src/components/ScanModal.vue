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
    <b-button @click="showCameraStream=!showCameraStream" class="btn-block">Scan QR with camera <b-icon icon="camera" /></b-button>
    <div v-if="showCameraStream" class="border border-secondary rounded m-2">
      <qrcode-stream @decode="onDecode"></qrcode-stream>
    </div>
   <!--  <b-overlay       
        :show="isSubmitBusy"
        rounded
        opacity="0.3"
        spinner-small
        spinner-variant="primary"
        >
      </b-overlay>-->
        <b-button type="submit" variant="primary" class="btn-block">Submit</b-button>

    </b-form>
  </b-modal>
</template>
<script>
import { QrcodeStream } from 'vue-qrcode-reader'

export default {
    components:{
        //QrcodeCapture,
        QrcodeStream,
        //QrcodeDropZone,
    },
    data() { 
        return {
            form: {
                uid: ''
            },   
            showCameraStream: false,
            //isSubmitBusy: false,
        }
    },
    props:{
    },
    computed:{
        validation() {
            var uid = this.form.uid
            return this.validate(uid)
        },
    },
    methods: {
        validate(uid){
            return uid.length >= 4
        },
        onDecode(uid){
            this.form.uid = uid
            this.submit()
        },
        // onDetect(uid){
        //     this.form.uid = uid
        // },
        submit(){
            console.log('submit')
            let uid = this.form.uid;
            if(!this.validation)
                return //TODO error message
            //this.isSubmitBusy = true
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