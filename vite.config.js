import { defineConfig } from 'vite';
import { ViteRsw } from 'vite-plugin-rsw';

export default defineConfig({
  plugins: [
    ViteRsw(),
  ],
  esbuild: {
    supported: {
      'top-level-await': true
    }
  }
})
