import axios from 'axios'
import { authLogin, authLogout, authSignup, authMe } from '@/api/sdk.gen'
import { useUserStore } from '@/stores/user'

export async function login(username: string, password: string, stay_logged_in: boolean) {
	const userStore = useUserStore()
	const { data: user, error } = await authLogin({ 
		body: { 
			username: username, 
			password: password, 
			stay_logged_in: stay_logged_in 
		}, 
		withCredentials: true 
	})
	if(error) { return error }
	userStore.user = user
}

export async function logout() {
	const userStore = useUserStore()
	const { error } = await authLogout({ withCredentials: true })
	if(error) { console.log(error) }
	userStore.user = null
}

export async function me() {
	const userStore = useUserStore()
	const { data: user, error } = await authMe({ withCredentials: true })
	if(error) { return error }
	userStore.user = user
}

export async function signup(username: string, password: string) {
	const { error } = await authSignup({ body: { username: username, password: password }, withCredentials: true })
	if(error) { return error }
}
