<template>
<div>
  <div class="container">
    <b-form @submit="onSubmit" @reset="onReset">
      <b-form-group
        validated
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

      <b-form-group id="input-group-2" :label="password_label" label-for="password-input" novalidate>
        <b-form-input
          id="password-input"
          v-model="form.password"
          type="password"
          required
          placeholder="Password"
          aria-describedby="password-help-block"
          :state="validation"
        ></b-form-input>
        <b-form-text id="password-help-block">
        Your password must be 8-20 characters long.
        </b-form-text>
        <b-form-invalid-feedback :state="validation">Weak password.</b-form-invalid-feedback>
        <b-form-valid-feedback :state="validation">Looks Good.</b-form-valid-feedback>
      </b-form-group>

      <b-form-group id="remember-me-group" novaldate>
        <b-form-checkbox-group v-model="form.remember" id="checkboxes-4">
          <b-form-checkbox value="remember-me">Remember me</b-form-checkbox>
        </b-form-checkbox-group>
      </b-form-group>
      <b-button type="submit" variant="primary" class="btn-block">Submit</b-button>
        <b-alert
          :show="wrongCredentialsAlertCountdown"
          dismissible
          fade
          class="position-fixed fixed-bottom m-0 rounded-0"
          style="z-index: 2000;"
          variant="danger"
          @dismiss-count-down="wrongCredentialsAlertCountdown=$event"
        >
          Wrong credentials. Try again.
        </b-alert>
        <b-alert
          :show="successfulLoginAlertCountdown"
          dismissible
          fade
          class="position-fixed fixed-bottom m-0 rounded-0"
          style="z-index: 2000;"
          variant="success"
          @dismiss-count-down="successfulLoginAlertCountdown=$event"
        >
          Succesfully logged in.
        </b-alert>
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
        isRegistration: this.propRegistration,
        wrongCredentials: false,
        wrongCredentialsAlertCountdown: 0,
        successfulLoginAlertCountdown:0,
      }
    },
    computed:{
      password_label() {
          return this.isRegistration?"Your new password:":"Your password:";
      },
      switch_text(){
          return this.isRegistration?
           {text: "Already registered?", button: "Log-in"}:
            {text: "Not registered?", button: "Sign-up"};
      },
      validation() {
        var password = this.form.password
        return password.length >= 8 && password.length <=20
      },
    },
    methods: {
      switchAction(){
        this.isRegistration = !this.isRegistration
        this.$emit('switch-action', this.isRegistration);
      },
      async onSubmit(evt) {
        evt.preventDefault();
        if(!this.validateForm)
          return
        var wasRegistration = this.isRegistration;
        let endpoint = this.isRegistration?"/register":"/login"
        this.$api.post(endpoint, {
          email: this.form.email,
          password: this.form.password,
          remember: this.form.remember,
        })
        .then(res => {
          if(wasRegistration){
              let code = res.data;
              console.log(code);
              //BEGIN temp validation
              let url = this.$api.defaults.baseURL+"/register/confirm?code=" 
              + encodeURIComponent(code);
              let win = window.open(url, '_blank');
              win.focus();
              //END temp
              this.$emit('successful-registration');

          }else{
            //login
            if(res.status == '200'){
              this.$emit('successful-login')
              this.showSuccessfulLoginAlert()
              this.$store.commmit('logged_in')
            }
          }
        }).catch( (err) => {
          if(err.response){
            if(err.response.status == '401'){
              this.wrongCredentials = true;
              this.form.password = '';
              this.showWrongCredentialsAlert();
            }
          }
        });
      },
      onReset(evt) {
        evt.preventDefault()
        this.resetForm()
      },
      resetForm(){
        this.form.email = ''
        this.form.password  = ''
        this.form.remember = []
      },
      validateForm(){
        return true;
      },
      showWrongCredentialsAlert(){
        this.wrongCredentialsAlertCountdown = 3
      },      
      showSuccessfulLoginAlert(){
        this.successfulLoginAlertCountdown = 3
      }
    }
  }
</script>
<style scoped>
button.switch-action{
    width: 6rem;
}
</style>