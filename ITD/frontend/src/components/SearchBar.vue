<template>
<div class="pt-4">
   <!--<b-input-group  size="lg" id="input-group-search" @change="$emit('submit')">
      <b-form-input type="search" id="input-search" placeholder="Search a store"></b-form-input>
        <b-input-group-append is-text>
            <b-icon icon="search"></b-icon>
        </b-input-group-append>
  </b-input-group>-->
  <Autocomplete
    class="input-search"
    :search="search"
    placeholder="Search a store"
    aria-label="Search a store"
    :get-result-value="getResultValue"
    auto-select
    @submit="onSubmit"
    ref="autocomplete"
    :debounce-time="100"
    ></Autocomplete>    
</div>
</template>

<script>
export default {
  components: {

  },
  data(){ return {

  }},
  methods: {
    // search(input) {
    //   const url = `${wikiUrl}/w/api.php?${params}&srsearch=${encodeURI(input)}`

    //   return new Promise((resolve) => {
    //     if (input.length < 3) {
    //       return resolve([])
    //     }

    //     fetch(url)
    //       .then((response) => response.json())
    //       .then((data) => {
    //         resolve(data.query.search)
    //       })
    //   })
    // },
    search(input) {
      if(input === "") return []
      else{
        return this.$api.get("/search?q="+input)
        .then( (res) => {
          return res.data;
        })
      }
    },
    getResultValue(result){
        return result.name;
    },
    onSubmit(result){
        this.$emit('submit', result);
        console.log("emit selected: " + result.name);
    },
    reset(){
        this.$refs.autocomplete.value = ''
    }
  }

}
</script>

<style>

</style>