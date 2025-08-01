<script setup lang="ts">
	import { computed, defineModel } from 'vue'

	const model = defineModel<string>()
	defineOptions({ inheritAttrs: false })
	const props = withDefaults(
		defineProps<{
			placeholder?: string  // mark as optional in the type
			label?: string
			hint?: string
			error?: string
			required?: boolean
		}>(),
		{
			placeholder: '',      // ← default values
			label: '',
			hint: '',
			error: '',
			required: false,
		},
	)

	let uid = 0						// Unique per module, shared across instances
	function useId() {
		return `tf-${uid++}`
	}

	// Generate Ids
	const inputId = useId()
	const hintId = `${inputId}-hint`
	const errId = `${inputId}-err`

	// aria-describedby resolves to "" if neither hint nor error exists
	const described = computed(() => {
		const ids = []
		if (props.hint)  ids.push(hintId)
		if (props.error) ids.push(errId)
		return ids.join(' ') || undefined
	})
</script>

<template>
	<div class="flex flex-col gap-1 text-sm">
		<label :for="inputId" class="font-medium">
			<span v-if="required && label" aria-hidden="true" class="text-red-600">*</span>
			{{ label }}
		</label>
		<input type="text" 
					 :id="inputId"
					 :placeholder="required ? `* ${placeholder}` : placeholder"
					 :aria-required="required || undefined"
					 :aria-invalid="Boolean(error) || undefined"
					 :aria-describedby="described"
					 v-model="model"
					 v-bind="{ ...$attrs, value: model }" @input="model = ($event.target as HTMLInputElement).value"
					 class="rounded border px-3 py-2 focus:ring-2 focus:ring-indigo-500 disabled:opacity-50"
		/>
		<p v-if="hint && !error" :id="hintId" class="text-gray-500">
			{{ hint }}
		</p>
		<p v-if="error" :id="errId" class="text-red-600">
			{{ error }}
		</p>
	</div>
</template>
