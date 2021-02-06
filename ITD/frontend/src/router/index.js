import Vue from 'vue'
import VueRouter from 'vue-router'
import Home from '../views/Home.vue'
import Search from '../views/Search.vue'
import Tokens from '../views/Tokens.vue'
import Staff from '../views/Staff.vue'
import StaffHome from '../views/StaffHome.vue'
import StaffTokens from '../views/StaffTokens.vue'
// DEV
import Dev from '../views/dev/Dev.vue'
import DevIds from '../views/dev/DevIds.vue'
import DevNewStaff from '../views/dev/DevNewStaff.vue'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    component: Home,
    children: [{
      path: '',
      component: Search
    },
    {
      path: 'tokens',
      component: Tokens
    }, 
    {
      path: 'tokens/:uid',
      component: Tokens
    },]
  },
  {
    path: '/staff',
    // lazy example
    //component: () => import(/* webpackChunkName: "about" */ '../views/Staff.vue')
    component: Staff,
    children: [{
        path: '',
        component: StaffHome
      },
      {
        path: 'tokens/:uid',
        component: StaffTokens
      },
    ]
  },
  {
    path: '/dev',
    // lazy example
    //component: () => import(/* webpackChunkName: "about" */ '../views/Staff.vue')
    component: Dev,
    children: [{
        path: '',
        component: DevIds
      },
      {
        path: 'new_staff',
        component: DevNewStaff
      },
    ]
  },
]

const router = new VueRouter({
  mode: 'history',
  routes
})

export default router
