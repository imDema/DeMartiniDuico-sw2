<template>
<b-modal ref="qrmodal" hide-footer>
    <template #modal-title>
        Show the QR to the Staff
    </template>
    <div class="px-auto">
    <div style="max-width: 280px;">
    <qriously :value="value" :size="280"/>
    <div class="qr-mono">{{value}}</div>
    </div>
    </div>
</b-modal>
</template>
<script>
export default {
    props: {
        value: String
    },
    methods: {
        show(){
            this.$refs.qrmodal.show()
        }
    }
}
</script>
<style scoped>
    .qr-mono{
        font-family: 'Courier New', Courier, monospace;
        font-size: 1.5rem;
        text-align: center;
    }
</style>