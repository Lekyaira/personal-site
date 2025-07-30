import api from '@/axios';             // <- the one with the Pinia interceptor
import type { CreateClientConfig } from '@/api/client/client.gen';

export const createClientConfig: CreateClientConfig = (defaults) => ({
  ...defaults,
  // Tell the SDK to *reuse* our instance
	axios: api,
  // If you generated “auth” schemes you can also fill them here:
  // auth: () => useAuthStore().token,
});
