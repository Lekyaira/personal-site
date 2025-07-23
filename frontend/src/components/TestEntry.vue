<script setup lang="ts">
import { Test } from '@/api/types.gen';

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
}
</style>
