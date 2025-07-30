<script setup lang="ts">
	import TestEntry from '@/components/TestEntry.vue';
	import { listTestEntries } from '@/api/sdk.gen';

	const { data: entries } = await listTestEntries();
</script>

<template>
	<div v-for="entry in entries" :key="entry.id">
		<TestEntry :entry="entry" />
	</div>
</template>
