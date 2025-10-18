export default {
  input:  'http://localhost:8000/openapi.json',   // Rocket dev URL
  output: 'src/api',                              // any folder you like
  plugins: [
		{
			name: '@hey-api/client-axios',
			runtimeConfigPath: './src/runtime.ts',
		},
	],             // tell it to emit Axios code
};
