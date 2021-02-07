<template>
<div>
  <div v-for="daily_schedule in weekly_schedule" :key="daily_schedule.dow">
      {{dowToString(daily_schedule.dow)}}: {{daily_schedule.open}} - {{daily_schedule.close}}
  </div>
</div>
</template>
<script>
export default {
    props:{
        weekly_schedule: Array,
    },
    methods:{
      dowToString(dow){
        let name = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday']
        if(dow>=1 && dow<=7){
          return name[dow-1]
        }else{
          return ""
        }
      } 
    }
}
</script>