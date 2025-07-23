export default {
  input:  'http://localhost:8000/openapi.json',   // Rocket dev URL
  output: 'src/api',                              // any folder you like
  plugins: ['@hey-api/client-axios'],             // tell it to emit Axios code
};
