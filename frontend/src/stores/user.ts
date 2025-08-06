import { defineStore } from 'pinia'
import { authLogin, authMe } from '@/api/sdk.gen'

export const useUserStore = defineStore('user', {
  state: () => ({
    user: null as null | { username: string, role: string }, 
  }),

  getters: {
    isAuthenticated: (state) => !!state.user,
		isAdmin: (state) => true, // TODO: Actually check user role
  },

  actions: {
		async refresh() {
			const { data: user_data, error } = await authMe({ withCredentials: true })
			if(error) { 
				this.user = null
				return error 
			}
			this.user = user_data
		},
  },

  // <-- Persistence config
  persist: {
    key: 'com.andersonryan-auth',
    paths: ['user'],  				          // only these fields are saved
    storage: sessionStorage,            // or localStorage / cookies
  },
})
