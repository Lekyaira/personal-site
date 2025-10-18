<script setup lang="ts">
	import { Teleport, onMounted, onUnmounted, watch } from 'vue'
	import { X } from 'lucide-vue-next'

	const props = defineProps<{
		// Controls visibility
		modelValue: boolean
	}>()

	const emit = defineEmits<{
		(e: 'update:modelValue', value: boolean): void
	}>()

	let previousActive: HTMLElement | null = null
	function close() { emit('update:modelValue', false) }

	onMounted(() => {
		watch(
			() => props.modelValue,
			(open) => {
				if (open) {
					previousActive = document.activeElement as HTMLElement
					document.body.style.overflow = 'hidden' // scroll-lock
					window.addEventListener('keydown', onKey)
					// send focus to the dialog after next tick
					requestAnimationFrame(() =>
						(document.getElementById('simple-modal') as HTMLElement)?.focus()
					)
				} else {
					document.body.style.overflow = ''
					window.removeEventListener('keydown', onKey)
					previousActive?.focus()
				}
			},
			{ immediate: true }
		)
	})

	// Clean up when removed
	onUnmounted(() => {
		document.body.style.overflow = ''
		window.removeEventListener('keydown', onKey)
	})

	// Close the dialog
	function onKey(e: KeyboardEvent) {
		if (e.key === 'Escape') close()
	}
</script>

<template>
  <!-- Only render when open -->
  <Teleport to="body" v-if="modelValue">
    <!-- Backdrop -->
    <div
      class="fixed inset-0 bg-black/30 backdrop-blur-sm"
      aria-hidden="true"
    />

    <!-- Positioner (centers content) -->
    <div
      class="fixed inset-0 flex items-center justify-center p-4 z-[1000]"
			@click.self="close"
      role="dialog"
      aria-modal="true"
    >
      <!-- Panel -->
      <div
        id="dialog-modal"
        tabindex="-1"
        class="relative w-full max-w-md rounded-lg bg-zinc-800 p-6 text-gray-100 shadow-xl outline-none"
      >
        <!-- close button -->
        <div
          @click="close"
          class="absolute right-3 top-3 grid h-8 w-8 place-items-center rounded-md bg-stone-900 text-white border border-transparent hover:border-indigo-500 transition duration-300 ease-in-out"
          aria-label="Close"
        >
					<X />
        </div>

        <!-- slot content -->
				<slot />
      </div>
    </div>
  </Teleport>
</template>
