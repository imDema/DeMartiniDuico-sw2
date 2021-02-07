<template>
<div class="container my-4">
    {{message}}
    <router-link to="/">Go back</router-link>
</div>
</template>
<script>
export default {
    data(){
        return {
            message: '',
        }
    },
    methods:{
        
    },
    created(){
        let code = this.$route.query.code
        if(!code)
            alert("Invalid code in GET query")
        this.$api.get("/register/confirm?code="+code)
        .then(() => {
            this.message = "Successful registration"
        })
        .catch( (err) => {
            this.message = err
        })
    }
}
</script>