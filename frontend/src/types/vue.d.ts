import { ComponentCustomProperties } from 'vue'

// Let TS know this file only adds types
export {}

declare module '@vue/runtime-core' {
  interface ComponentCustomProperties {
    $api: typeof import('@/api/client.gen').client
  }
}
