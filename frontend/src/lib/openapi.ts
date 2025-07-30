import { client } from '@/api/client.gen';   // already configured by runtime.ts
import type { App } from 'vue';

export default {
  install(app: App) {
    app.config.globalProperties.$api = client;
  },
};
