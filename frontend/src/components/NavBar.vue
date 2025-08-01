<script setup lang="ts">
	import { useRoute } from 'vue-router'
	import { useAuthStore } from '@/stores/auth'
	import Dialog from '@/components/Dialog.vue'
	import TextField from '@/components/TextField.vue'
	import { EyeIcon, EyeOffIcon } from 'lucide-vue-next'
	import { ref, reactive } from 'vue'

	const auth = useAuthStore()
	const route = useRoute()

	const login_open = ref(false)
	const login_form = reactive({
		username: '',
		password: '',
	})

	const menu_items = [
		{ name: 'Home', to: '/' },
	]

	if(auth.isAuthenticated) {
		menu_items.push({ name: 'Admin', to: '/admin' })
	}

	function login() {
		console.log('Username: ', login_form.username)
		console.log('Password: ', login_form.password)
		login_open = false
	}
</script>

<template>
	<!-- Log in dialog -->
		<Dialog v-model="login_open">
			<h2 class="text-xl font-bold mb-4">Log In</h2>
			<div class="p-2 m-1">
				<TextField v-model="login_form.username" placeholder="Email Address" :required="true"/>
				<input type="password" v-model="login_form.password" />
			</div>
			<button @click="login">Log In</button>
		</Dialog>
	<!-- End Log in dialog -->
	<!-- Nav bar -->
	<header class="absolute top-0 left-0 w-full">
		<div class="mx-4 my-2 border-b">
			<nav class="flex">
				<!-- Nav links -->
				<div class="mx-1 my-2 px-1 border-2 rounded-md hover:border-teal-300">
					<RouterLink v-for="item in menu_items" :key="item.name" :to="item.to" 
											class="transition-colors hover:text-teal-300" 
											:class="{ 'text-teal-400 font-semibold': route.path === item.to }"
					>
						{{ item.name }}
					</RouterLink>
				</div>
				<!-- End Nav links -->
				<!-- Log in button -->
				<button v-if="!auth.isAuthenticated" @click="() => (login_open = true)">Log In</button>
			</nav>
		</div>
	</header>
	<!-- End Nav bar -->
</template>

<style scoped>
.close-button button {
	width: 8;
	height: 8;
}
</style>
