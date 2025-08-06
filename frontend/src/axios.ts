import axios from 'axios'
// import { useAuthStore } from '@/stores/auth'

const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL,
})

// api.interceptors.request.use((config) => {
//   // we can’t import the store at top-level because of SSR,
//   // so we grab it lazily:
//   const auth = useAuthStore()
// 	if (auth.token) config.headers.Authorization = `Bearer ${auth.token}`
//   return config
// })

export default api
