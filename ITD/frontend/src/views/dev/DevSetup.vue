<template>
    <div>
        {{res}}
    </div>
</template>
<script>
export default {
    data(){
        return {
            res:''
        }
    },
    methods:{
        loadData(){
            return this.$api.get("/dev/setup_env")
            .then((response) => {
                this.res = response.data
            })
            .catch( (err) => {
                this.res = err
            })
        }
    },
    created(){
        this.loadData()
    }
}
</script>