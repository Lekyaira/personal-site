<script setup lang="ts">
defineProps<{ entry: Test }>()
</script>

<template>
	<div class="entry-block">
		<h3>{{ entry.title }}</h3>
		<p>{{ entry.body }}</p>
	</div>
</template>

<style scoped>
.entry-block {
	display: flex;
	flex-direction: column;
	border: 1px solid;
	margin: 1em 0 0 0;
}

h3 {
	font-size: 1.5em;
	font-weight: bold;
	border-bottom: 1px solid;
}

p {
	margin: 1em;
}
</style>
