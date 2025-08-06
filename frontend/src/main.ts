import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import { createPinia } from 'pinia'
import piniaPersist from 'pinia-plugin-persistedstate'
import { client } from '@/api/client.gen'
import OpenApiPlugin from '@/lib/openapi'
import { router } from '@/router'
import { useUserStore } from '@/stores/user'

// Set up OpenAPI bindings
client.setConfig({
	baseURL: import.meta.env.VITE_API_URL,
  withCredentials: true,        // any Axios option is allowed
  timeout: 10_000,
})

// Set up data store
const pinia = createPinia()
pinia.use(piniaPersist)

// Mount application
createApp(App).use(pinia).use(OpenApiPlugin).use(router).mount('#app')

// Initialize the user data
const userStore = useUserStore()
userStore.refresh()
