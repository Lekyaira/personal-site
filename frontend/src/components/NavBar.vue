<script setup lang="ts">
	import { useRouter, useRoute } from 'vue-router'
	import { useAuthStore } from '@/stores/auth'
	import Dialog from '@/components/Dialog.vue'
	import TextField from '@/components/TextField.vue'
	import { ref, reactive } from 'vue'
	import { LoginRequest } from '@/api/types.gen'
	import { authLogin } from '@/api/sdk.gen'

	const auth = useAuthStore()
	const route = useRoute()
	const router = useRouter()

	const login_open = ref(false)
	const login_form = reactive({
		username: '',
		password: '',
	})

	interface NavItem {
		name: string,
		to: string,
	}
	const guest_menu: NavItem[] = [
		{ name: 'Home', to: '/' },
	]
	const authorized_menu = ref<NavItem[]>([...guest_menu])

	async function login() {
		//let { data: token } = await authLogin({ body: {username: login_form.username, password: login_form.password }})
		//console.log('Token: ', token)
		let err = await auth.login(login_form.username, login_form.password)
		// Whether we succeeded or failed, empty the password field
		login_form.password = ''
		if(err) { 
			console.log('Error: ', err) 
			return
		}

		// Pull authoried pages and add them to the nav menu
		authorized_menu.value.push({ name: 'Admin', to: '/admin' }) // Pull from database based on user role

		// Close the login dialog, we're done with it
		login_open.value = false

		// Move the user to the home page
		router.push("/")
	}

	async function logout() {
		await auth.logout()
		authorized_menu.value = [...guest_menu]
		// Move the user to the home page
		router.push("/")
	}
</script>

<template>
	<!-- Log in dialog -->
		<Dialog v-model="login_open">
			<h2 class="text-xl font-bold mb-4">Log In</h2>
			<div class="p-2 m-1">
				<TextField v-model="login_form.username" placeholder="Email Address" :required="true"/>
				<TextField v-model="login_form.password" placeholder="Password" :required="true" :password="true"/>
			</div>
			<button @click="login">Log In</button>
		</Dialog>
	<!-- End Log in dialog -->
	<!-- Nav bar -->
	<header class="absolute top-0 left-0 w-full">
		<div class="mx-4 my-2 border-b">
			<nav class="flex">
				<!-- Nav links -->
				<!-- TODO: Move this to a component -->
				<div v-for="item in authorized_menu" class="mx-1 my-2 px-1 border-2 rounded-md hover:border-teal-300">
					<RouterLink :key="item.name" :to="item.to" 
											class="transition-colors hover:text-teal-300" 
											:class="{ 'text-teal-400 font-semibold': route.path === item.to }"
					>
						{{ item.name }}
					</RouterLink>
				</div>
				<!-- End Nav links -->
				<!-- Log in button -->
				<button v-if="!auth.isAuthenticated" @click="() => (login_open = true)">Log In</button>
				<button v-if="auth.isAuthenticated" @click="logout">Log Out</button>
			</nav>
		</div>
	</header>
	<!-- End Nav bar -->
</template>
