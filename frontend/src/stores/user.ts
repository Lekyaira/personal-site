import { defineStore } from 'pinia'
import { authLogin, authMe, authLinks } from '@/api/sdk.gen'

export const useUserStore = defineStore('user', {
  state: () => ({
    user: null as null | { username: string, role: string }, 
		links: [{ name: 'Home', to: '/' }],
  }),

  getters: {
    isAuthenticated: (state) => !!state.user,
  },

  actions: {
		async refresh() {
			// Update user data and refresh logged in expiration
			const { data: user_data, me_error } = await authMe({ withCredentials: true })
			if(me_error) { 
				this.user = null
				return me_error 
			}
			this.user = user_data
			
			// Update links
			const { data: user_links, links_error } = await authLinks({ withCredentials: true })
			if(links_error) {
				this.links = [{ name: 'Home', to: '/' }]
				return links_error
			}
			this.links = user_links
		},
  },

  // <-- Persistence config
  persist: {
    key: 'com.andersonryan-user',
    paths: ['user'],  				          // only these fields are saved
    storage: sessionStorage,            // or localStorage / cookies
  },
})
