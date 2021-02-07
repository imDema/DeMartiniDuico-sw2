<template>
<div class="my-2">
  <div class="border-bottom d-flex justify-content-between  pb-2">
    <div class="h4">{{modal_header}}</div>
    <div v-if="!staff" class="d-flex flex-row justify-content-end align-items-center">
      <div class="p-2">{{switch_text.text}}</div>
      <b-button variant="outline-primary" class="switch-action" @click="switchAction">{{switch_text.button}}</b-button>
    </div>
  </div>
  <b-overlay :show="showOverlay" rounded="sm">
  <div class="container my-4">
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
        Your password must be at least 10 characters long.
        </b-form-text>
        <div v-if="isRegistration" class="mb-4">
        <b-form-invalid-feedback :state="validation">Weak password.</b-form-invalid-feedback>
        <b-form-valid-feedback   :state="validation">Looks Good.</b-form-valid-feedback>
        </div>
      </b-form-group>

      <b-form-group v-show="!isRegistration" id="remember-me-group" novaldate>
        <b-form-checkbox-group v-model="form.remember" id="checkboxes-4">
          <b-form-checkbox value="remember-me">Remember me</b-form-checkbox>
        </b-form-checkbox-group>
      </b-form-group>
      <b-overlay       
        :show="isSubmitBusy"
        rounded
        opacity="0.3"
        spinner-small
        spinner-variant="primary"
        >
        <b-button type="submit" variant="primary" class="btn-block">Submit</b-button>
      </b-overlay>
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
          :show="accountAlreadyExistsCountdown"
          dismissible
          fade
          class="position-fixed fixed-bottom m-0 rounded-0"
          style="z-index: 2000;"
          variant="danger"
          @dismiss-count-down="accountAlreadyExistsCountdown=$event"
        >
          This email is already in use. Log-in instead.
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
        <b-alert
          :show="successfulRegistrationAlertCountdown"
          dismissible
          fade
          class="position-fixed fixed-bottom m-0 rounded-0"
          style="z-index: 2000;"
          variant="success"
          @dismiss-count-down="successfulRegistrationAlertCountdown=$event"
        >
          Succesfully signed up. Check your email inbox.
        </b-alert>
    </b-form>
  </div>
  </b-overlay>
</div>
</template>

<script>
  export default {
    props: {
      propRegistration:{
        type: Boolean,
        default: false,
      },
      staff: Boolean,
    },
    data() {
      return {
        form: {
          email: '',
          password: '',
          remember: [],
        },
        isRegistration: this.propRegistration,
        wrongCredentialsAlertCountdown: 0,
        accountAlreadyExistsCountdown: 0,        
        successfulLoginAlertCountdown:0,
        successfulRegistrationAlertCountdown: 0,
        showOverlay: false,
        isSubmitBusy: false,
      }
    },
    computed:{
      password_label() {
          return this.isRegistration?"Your new password:":"Your password:";
      },
      modal_header() {
          return this.isRegistration?"Sign-up":"Log-in"
      },
      switch_text(){
          return this.isRegistration?
           {text: "Already registered?", button: "Log-in"}:
            {text: "Not registered?", button: "Sign-up"};
      },
      validation() {
        var password = this.form.password
        return password.length >= 8 && password.length <=256
      },
    },
    watch: {
      propRegistration(newValue) {
        this.isRegistration = newValue
      }
    },
    methods: {
      switchAction(){
        if(this.staff){
          return
        }
        //this.isRegistration = !this.isRegistration
        this.$emit('switch-action', !this.isRegistration)
        this.showOverlay = true
        setTimeout( () => {this.showOverlay = false}, 300)
      },
      async onSubmit(evt) {
        evt.preventDefault();
        if(!this.validateForm)
          return
        this.isSubmitBusy = true;
        var wasRegistration = this.isRegistration;
        var email = this.form.email
        let endpoint;
        if(this.staff){
          endpoint = "/staff/login"
        }else{
          endpoint = this.isRegistration?"/register":"/login"
        }
        this.$api.post(endpoint, {
          email: email,
          password: this.form.password,
          remember: this.form.remember !== [],
        })
        .then(res => {
          if(wasRegistration){
              let code = res.data;
              //BEGIN temp validation
              let url = window.location.origin + "/confirm?code="+ encodeURIComponent(code);
              console.log(url);
              alert("Open this url to confirm your registration: \n"+url);
              //END temp
              this.showSuccessfulRegistrationAlert();
              this.$emit('successful-registration');
          }else{
            //login
            //if(res.status == '200'){
              if(this.staff){
                this.$store.commit('staff_logged_in')
                this.$store.state.staff.email = email
              }else{
                this.$store.commit('logged_in')
                this.$store.state.customer.email = email
              }
              this.showSuccessfulLoginAlert()
              this.$emit('successful-login')
            //}
          }
        }).catch( (err) => {
          if(err.response){
            if(err.response.status == '401'){
              this.form.password = '';
              this.showWrongCredentialsAlert();
            }else if(err.response.status == '400'){
              if(err.response.data == 'Account already exists'){
                this.showAccountAlreadyExistsAlert();
              }else{
                this.showWrongCredentialsAlert()
              }
            }else{
              this.$emit('connection-failure')
            }
          }else{
            this.$emit('connection-failure')
          }
        }).finally( () => {
          setTimeout( () => {this.isSubmitBusy = false}, 250)
        })
      },
      onReset(evt) {
        evt.preventDefault()
        this.resetForm()
      },
      resetForm(){
        this.form.email = ''
        this.form.password  = ''
        this.form.remember = false
      },
      validateForm(){
        return true;
      },
      showWrongCredentialsAlert(){
        this.wrongCredentialsAlertCountdown = 3
      },
      showAccountAlreadyExistsAlert(){
        this.accountAlreadyExistsCountdown = 3
      },    
      showSuccessfulLoginAlert(){
        this.successfulLoginAlertCountdown = 3
      },
      showSuccessfulRegistrationAlert(){
        this.successfulRegistrationAlertCountdown = 3
      }
    }
  }
</script>
<style scoped>
button.switch-action{
    width: 6rem;
}
</style>