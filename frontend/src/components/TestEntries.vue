<script setup lang="ts">
	import TestEntry from '@/components/TestEntry.vue';
	import { listTestEntries } from '@/api/sdk.gen';
	const res = await listTestEntries();
	const data = res.data;
</script>

<template>
	<ul v-if="data">
		<li v-for="entry in data"><TestEntry :entry="entry" /></li>
	</ul>
</template>
