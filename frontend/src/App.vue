<script setup lang="ts">
	import NavBar from '@/components/NavBar.vue'
</script>

<template>
	<NavBar />
	<main>
		<Suspense>
			<RouterView />
			<template #fallback>
				<div>Loading...</div>
			</template>
		</Suspense>
	</main>
</template>

<style scoped>
</style>
