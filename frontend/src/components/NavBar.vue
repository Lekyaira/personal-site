<script setup lang="ts">
	import { useRouter, useRoute } from 'vue-router'
	import { useUserStore } from '@/stores/user'
	import { login, logout } from '@/lib/auth'
	import Dialog from '@/components/Dialog.vue'
	import TextField from '@/components/TextField.vue'
	import { ref, reactive } from 'vue'
	import { authLinks } from '@/api/sdk.gen'

	const userStore = useUserStore()
	const route = useRoute()
	const router = useRouter()

	const login_open = ref(false)
	const login_form = reactive({
		username: '',
		password: '',
		stay_logged_in: false,
	})

	async function login_submit() {
		let error = await login(login_form.username, login_form.password, login_form.stay_logged_in)
		// Whether we succeeded or failed, empty the password field
		login_form.password = ''
		if(error) { 
			console.log('Error: ', error) 
			return
		}

		// Pull authoried pages and add them to the nav menu
		userStore.refresh()

		// Close the login dialog, we're done with it
		login_open.value = false

		// Move the user to the home page
		router.push("/")
	}

	async function logout_submit() {
		await logout()
		// Save login state
		userStore.refresh()
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
				<div class="flex flex-row gap-1">
					<span>Stay logged in?</span>
					<input type="checkbox" v-model="login_form.stay_logged_in"/>
				</div>
			</div>
			<button @click="login_submit">Log In</button>
		</Dialog>
	<!-- End Log in dialog -->
	<!-- Nav bar -->
	<header class="absolute top-0 left-0 w-full">
		<div class="mx-4 my-2 border-b">
			<nav class="flex">
				<!-- Nav links -->
				<!-- TODO: Move this to a component -->
				<div v-for="item in userStore.links" class="mx-1 my-2 px-1 border-2 rounded-md hover:border-teal-300">
					<RouterLink :key="item.name" :to="item.to" 
											class="transition-colors hover:text-teal-300" 
											:class="{ 'text-teal-400 font-semibold': route.path === item.to }"
					>
						{{ item.name }}
					</RouterLink>
				</div>
				<!-- End Nav links -->
				<!-- Log in button -->
				<button v-if="!userStore.isAuthenticated" @click="() => (login_open = true)">Log In</button>
				<button v-if="userStore.isAuthenticated" @click="logout_submit">Log Out</button>
			</nav>
		</div>
	</header>
	<!-- End Nav bar -->
</template>
