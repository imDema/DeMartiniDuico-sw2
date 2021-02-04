<template>
    <b-modal id="login-modal" no-close-on-esc no-close-on-backdrop hide-header hide-footer busy>
    <LoginForm id="login-form" :prop-registration="isRegistration" @switch-action="isRegistration=$event" @successful-login="slowClose" @successful-registration="isRegistration=false" @connection-failure="$emit('connection-failure')"/>  
    <template #modal-header>
        <div class="modal_header">{{modal_header}}</div>
    </template>
  </b-modal>
</template>
<script>
import LoginForm from './LoginForm'
export default {
    components:{
        LoginForm
    },
    data() { 
        return {
            isRegistration: false,
        }
    },
    props:{
    },
    computed:{
        modal_header() {
            return this.isRegistration?"Sign-up":"Log-in"
        },
    },
    methods: {
        close(){
            this.$bvModal.hide('login-modal');
        },
        slowClose(){
            setTimeout(this.close, 500);
        }
    }
}
</script>
<style scoped>
.modal_header{
    font-size: 1.5rem;
}
</style>