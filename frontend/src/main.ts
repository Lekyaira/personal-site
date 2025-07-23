import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import { createPinia } from 'pinia'
import { client } from '@/api/client.gen';

// Set up OpenAPI bindings
client.setConfig({
  baseURL: import.meta.env.VITE_API_URL || 'http://localhost:8000', // TODO: Set up dev/production configurations
  withCredentials: true,        // any Axios option is allowed
  timeout: 10_000,
});

// Initialize applicaton
const app = createApp(App)

// Set up data store
const pinia = createPinia()
app.use(pinia)

// Mount application
app.mount('#app')
