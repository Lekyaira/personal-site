import { defineStore } from 'pinia'
import axios from 'axios'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    token: '' as string,                // JWT
    user: null as null | { id: string; email: string }, // optional
  }),

  getters: {
    isAuthenticated: (state) => !!state.token,
    authHeader: (state) =>
      state.token ? { Authorization: `Bearer ${state.token}` } : {},
  },

  actions: {
    /** Call during login */
    async login(credentials: { email: string; password: string }) {
      const { data } = await axios.post('/login', credentials)
      this.token = data.token
    },

    /** Call when you want to remove credentials */
    logout() {
      this.$reset()                     // clears state
    },

    /** Persisted tokens sometimes expire – quick helper */
    async refresh() {
      try {
        const { data } = await axios.post('/refresh', null, {
          headers: this.authHeader,
        })
        this.token = data.token
      } catch (err) {
        this.logout()
      }
    },
  },

  // <-- Persistence config
  persist: {
    key: 'com.andersonryan-auth',
    paths: ['token'],           // only these fields are saved
    storage: sessionStorage,            // or localStorage / cookies
  },
})
