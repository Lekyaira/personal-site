import { createRouter, createMemoryHistory} from 'vue-router'
import Home from '@/pages/Home.vue'
import Admin from '@/pages/Admin.vue'
import NotFound from '@/pages/NotFound.vue'

const routes = [
  { path: '/', component: Home },
	{ path: '/admin', component: Admin },
  { path: '/:pathMatch(.*)*', component: NotFound },
]

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
})
