<template>
    <div>
    <nav class="m-4">
        <router-link to="/dev">Display Data</router-link> |
        <router-link to="/dev/new_staff">New staff</router-link>
        <small class="mx-2">Only for testing purposes</small>
    </nav>
    <div class="container">
        <router-view />
    </div>
    </div>
</template>
<script>
export default {
    
}
</script>