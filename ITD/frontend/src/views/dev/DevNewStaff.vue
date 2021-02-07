<template>
<div>
    <b-form @submit.prevent.stop="submitForm">
    <h5>Generate a new staff account</h5>
    <b-form-group id="input-group-1" label="Email:" label-for="input-1">
    <b-form-input
      id="input-1"
      v-model="form.email"
      type="email"
      placeholder="Enter email"
      required
    ></b-form-input>
    </b-form-group>
    <b-form-group id="input-group-shop" label="Store:" label-for="input-shop">
    <b-form-select
      id="input-shop"
      class="mb-2 mr-sm-2 mb-sm-0"
      v-model="form.shop"
      :options="shops"
    />
    </b-form-group>
    <b-form-group id="input-group-2" label="Password:" label-for="input-2">
      <b-form-input
      id="input-2"
      v-model="form.password"
      type="password"
      placeholder="Enter password"
      required
    ></b-form-input>
    </b-form-group>

    <b-button variant="primary" type="submit">Create staff account</b-button>
    </b-form>
</div>
</template>
<script>
export default {
    data(){
        return {
            shops: [],
            form:{
              email: '',
              password: '',
              shop:'',
            }
        }
    },
    methods: {
      submitForm(){
        console.log(this.form)
        let email = this.form.email
        let password = this.form.password
        let shop_id = this.form.shop
        this.$api.post("/dev/new_staff", 
          {
            email,
            password,
            shop_id,
          }
        )
        .then( () => {
          alert("Staff account successfully created!")
        })
        .catch( (err) => alert(err) )
        .finally(this.resetForm)
      },
      fetchShops(){
        this.$api.get("/dev/shops")
        .then((response) => {
          if(!response.data){
            alert("Failed to fetch shops")
            return
          }
          this.shops = response.data.map((s) => { return {text: s.name, value: s.uid}} )
        })
        .catch(() => {
          alert("Failed to fetch shops")
        })
      },
      resetForm(){
        this.form.email = ''
        this.form.password = ''
        this.form.shop = ''
      }
    },
    created(){
      this.fetchShops()
    }
}
</script>