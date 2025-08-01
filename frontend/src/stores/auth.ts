import { defineStore } from 'pinia'
import axios from 'axios'
import { authLogin, authRefreshToken } from '@/api/sdk.gen'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    token: '' as string,
    user: null as null | { id: string; email: string, role: string }, // TODO: Return user data with login. Role should be enum.
  }),

  getters: {
    isAuthenticated: (state) => !!state.token,
    authHeader: (state) =>
      state.token ? { Authorization: `Bearer ${state.token}` } : {},
		isAdmin: (state) => true, // TODO: Actually check user role
  },

  actions: {
    /** Call during login */
    async login(email: string, password: string) {
      const { data: token, error } = await authLogin({ body: { username: email, password: password }})
			if(error) {
				return error
			}
			this.token = token
    },

    /** Call when you want to remove credentials */
    logout() {
      this.$reset()                     // clears state
    },

    /** Persisted tokens sometimes expire – quick helper */
    async refresh() {
			const { data: token, error } = await authRefreshToken()
			if(error) {
				// Failed refresh, something is wrong.
				// Log out so the site will prompt the user to log in for a new token.
				this.logout()
				return
			}
			this.token = token 
      // try {
      //   const { data } = await axios.post('/refresh', null, {
      //     headers: this.authHeader,
      //   })
      //   this.token = data.token
      // } catch (err) {
      //   this.logout()
      // }
    },
  },

  // <-- Persistence config
  persist: {
    key: 'com.andersonryan-auth',
    paths: ['token'],           // only these fields are saved
    storage: sessionStorage,            // or localStorage / cookies
  },
})
