<script setup lang="ts">
	import { ref, computed, defineModel } from 'vue'
	import { EyeIcon, EyeOffIcon } from 'lucide-vue-next'

	const model = defineModel<string>()
	defineOptions({ inheritAttrs: false })
	const props = withDefaults(
		defineProps<{
			placeholder?: string  // mark as optional in the type
			label?: string
			hint?: string
			error?: string
			required?: boolean
			password?: boolean
		}>(),
		{
			placeholder: '',      // ← default values
			label: '',
			hint: '',
			error: '',
			required: false,
			password: false,
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

	// Show/hide the password
	const password_visible = ref(false)
</script>

<template>
	<div class="flex flex-col gap-1 text-sm">
		<label :for="inputId" class="font-medium">
			<span v-if="required && label" aria-hidden="true" class="text-red-600">*</span>
			{{ label }}
		</label>
		<div class="block relative w-full">
			<input :type="password ? password_visible ? 'text' : 'password' : 'text'" 
						 :id="inputId"
						 :placeholder="required ? `* ${placeholder}` : placeholder"
						 :aria-required="required || undefined"
						 :aria-invalid="Boolean(error) || undefined"
						 :aria-describedby="described"
						 v-model="model"
						 v-bind="{ ...$attrs, value: model }" @input="model = ($event.target as HTMLInputElement).value"
						 class="rounded border px-3 py-2 w-full focus:ring-2 focus:ring-indigo-500 disabled:opacity-50"
			/>
			<!-- Password visibility toggle eye icon -->
			<div class="absolute inset-y-0 right-2 grid place-items-center"
					 v-if="password"
					 aria-label="Toggle password visibility"
					 @click="password_visible = !password_visible"
			>
				<EyeOffIcon v-if="password_visible"/>
				<EyeIcon v-if="!password_visible"/>
			</div>
		</div>
		<p v-if="hint && !error" :id="hintId" class="text-gray-500">
			{{ hint }}
		</p>
		<p v-if="error" :id="errId" class="text-red-600">
			{{ error }}
		</p>
	</div>
</template>
