import Vue from 'vue'
import VueRouter from 'vue-router'
import Home from '../views/Home.vue'
import Search from '../views/Search.vue'
import Tokens from '../views/Tokens.vue'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home,
    children: [{
      path: '',
      component: Search
    }, {
      path: 'tokens',
      component: Tokens
    }]
  },
  {
    path: '/staff',
    name: 'Staff',
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () => import(/* webpackChunkName: "about" */ '../views/Staff.vue')
  }
]

const router = new VueRouter({
  mode: 'history',
  routes
})

export default router
