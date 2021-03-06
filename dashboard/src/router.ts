import Vue from 'vue'
import Router, { RouterOptions } from 'vue-router'
import Home from './views/Home'
import { routes as usersRoutes } from './views/users'
import { routes as rolesRoutes } from './views/roles'
import { routes as patsRoutes } from './views/pats'
import { routes as containersRoutes } from './views/containers'
import { routes as mavenRoutes } from './views/maven'
import About from './views/About'
import OAuthCallback from './views/OAuthCallback'
import { vuexOidcCreateRouterMiddleware } from 'vuex-oidc'
import store from './store'
import NotFound from './views/NotFound'

Vue.use(Router)

const opts: RouterOptions = {
  mode: 'history',
  routes: [
    { path: '/', name: 'home', component: Home },
    { path: '/about', name: 'about', component: About },
    { path: '/dashboard/auth/callback', name: 'oauthCallback', component: OAuthCallback, meta: { public: true } },
    ...usersRoutes,
    ...rolesRoutes,
    ...patsRoutes,
    ...containersRoutes,
    ...mavenRoutes,
    { path: '*', name: '404', component: NotFound }
  ]
};
const router = new Router(opts)

router.beforeEach(vuexOidcCreateRouterMiddleware(store))

export default router
