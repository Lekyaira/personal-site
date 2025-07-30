<script setup lang="ts">
	import TestEntries from '@/components/TestEntries.vue';
</script>

<template>
	<h1>Hello...</h1>
<Suspense>
  <!-- component with nested async dependencies -->
  <TestEntries />

  <!-- loading state via #fallback slot -->
  <template #fallback>
    Loading...
  </template>
</Suspense>
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
