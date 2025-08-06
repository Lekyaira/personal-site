import api from '@/axios';             // <- the one with the Pinia interceptor
import type { CreateClientConfig } from '@/api/client/client.gen';

export const createClientConfig: CreateClientConfig = (defaults) => ({
  ...defaults,
	axios: api,
});
