<template>
<div>
  <div class="container">
    <b-form @submit="onSubmit" @reset="onReset" v-if="show">
      <b-form-group
        id="input-group-1"
        label="Email address:"
        label-for="input-1"
        description="We'll never share your email with any Third Parties">
          <b-form-input
          id="input-1"
          v-model="form.email"
          type="email"
          required
          placeholder="Enter email"
        ></b-form-input>
      </b-form-group>

      <b-form-group id="input-group-2" :label="password_label" label-for="input-2">
        <b-form-input
          id="input-2"
          v-model="form.password"
          type="password"
          required
          placeholder="Password"
          aria-describedby="password-help-block"
        ></b-form-input>
        <b-form-text id="password-help-block">
        Your password must be 8-20 characters long.
        </b-form-text>
      </b-form-group>

      <b-form-group id="input-group-4">
        <b-form-checkbox-group v-model="form.remember" id="checkboxes-4">
          <b-form-checkbox value="remember-me">Remember me</b-form-checkbox>
        </b-form-checkbox-group>
      </b-form-group>

      <b-button type="submit" variant="primary" class="btn-block">Submit</b-button>
    </b-form>
  </div>
  <div class="border-top pt-2 mt-4 d-flex flex-row justify-content-end align-items-center">
    <div class="p-2">{{switch_text.text}}</div>
    <b-button variant="outline-primary" class="switch-action" @click="switchAction">{{switch_text.button}}</b-button>
  </div>
</div>
</template>

<script>
  export default {
    props: {
      propRegistration:{
        type: Boolean,
        default: false,
      }
    },
    data() {
      return {
        form: {
          email: '',
          password: '',
        },
        show: true,
        isRegistration: this.propRegistration,
      }
    },
    computed:{
      password_label() {
          return this.isRegistration?"Your new password:":"Your password:";
      },
      switch_text(){
          return this.isRegistration?
            {text: "Not registered?", button: "Sign-up"}:
           {text: "Already registered?", button: "Log-in"};
      },
    },
    methods: {
      switchAction(){
        this.isRegistration = !this.isRegistration
        this.$emit('switch-action', this.isRegistration);
      },
      async onSubmit(evt) {
        evt.preventDefault();
        let endpoint = this.isRegistration?"/register":"/login"
        this.$api.post(endpoint, {
          email: this.form.email,
          password: this.form.password,
          remember: this.form.remember,
        })
        .then(res => {
          console.log(res.data.args);
        }).catch(err => {
          console.log(err);
        });
      },
      onReset(evt) {
        evt.preventDefault()
        // Reset form values
        this.form.email = ''
        this.form.password  = ''
        this.form.remember = []
        // Trick to reset/clear native browser form validation state
        this.show = false
        this.$nextTick(() => {
          this.show = true
        })
      }
    }
  }
</script>
<style scoped>
button.switch-action{
    width: 6rem;
}
</style>