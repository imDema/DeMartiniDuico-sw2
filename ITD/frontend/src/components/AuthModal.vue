<template>
    <b-modal id="login-modal" no-close-on-esc no-close-on-backdrop hide-header-close hide-footer busy>
    <LoginForm id="login-form" :prop-registration="isRegistration" @switch-action="isRegistration=$event"/>  
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
    }
}
</script>
<style scoped>
.modal_header{
    font-size: 1.5rem;
}
</style>