import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Preprocessor for Tailwind CSS
  preprocess: vitePreprocess(),

  kit: {
    // Adapter for production build
    adapter: adapter(),

    // Add any custom configurations here
    alias: {
      $lib: 'src/lib',
      $components: 'src/lib/components'
    }
  }
};

export default config;