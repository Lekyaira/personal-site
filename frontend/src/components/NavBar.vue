<script setup lang="ts">
	import { useRoute } from 'vue-router'
	import { useAuthStore } from '@/stores/auth'
	import { Dialog } from '@ark-ui/vue/dialog'
	import { Field } from '@ark-ui/vue/field'
	import { PasswordInput } from '@ark-ui/vue/password-input'
	import { EyeIcon, EyeOffIcon, X } from 'lucide-vue-next'
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
	}
</script>

<template>
	<!-- Log in dialog -->
	<Dialog.Root v-model:open="login_open">
    <Teleport to="body">
		<!-- TODO: Move dialog styling to .css -->
      <Dialog.Backdrop class="fixed inset-0 bg-black/20 backdrop-blur-sm"/>
      <Dialog.Positioner class="fixed inset-0 flex items-center justify-center p-4">
        <Dialog.Content class="relative w-full max-w-md rounded-md bg-white p-6 shadow-xl dark:bg-gray-800">
					<!-- Log in form -->
          <Dialog.Title class="text-lg font-semibold">Log In</Dialog.Title>
          <Dialog.Description class="mb-4 text-sm text-gray-500">Enter your credentials</Dialog.Description>
					<input type="text" v-model="login_form.username" />
					<input type="password" v-model="login_form.password" />
					<Dialog.CloseTrigger class="absolute top-1 right-1 text-white close-button"><X /></Dialog.CloseTrigger>
					<button class="mt-6 rounded bg-teal-600 px-4 py-2 text-white" @click="login()">Log In</button>
					<!-- End Log in form -->
        </Dialog.Content>
      </Dialog.Positioner>
    </Teleport>
  </Dialog.Root>
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
