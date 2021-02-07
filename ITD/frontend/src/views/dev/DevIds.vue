<template>
    <div v-html="ids">
    </div>
</template>
<script>
export default {
    data(){
        return {
            ids:''
        }
    },
    methods:{
        loadData(){
            return this.$api.get("/dev/ids")
            .then((response) => {
                this.ids = response.data.replace(/\n/g, "<br />");
            })
            .catch( console.log )
        }
    },
    created(){
        this.loadData()
    }
}
</script>