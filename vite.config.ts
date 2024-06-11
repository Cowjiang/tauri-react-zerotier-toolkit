import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { viteStaticCopy } from 'vite-plugin-static-copy'

export default defineConfig(async () => ({
  base: './',
  plugins: [
    react(),
    viteStaticCopy({
      targets: [
        {
          src: 'src/__utools__/*',
          dest: ''
        }
      ]
    })
  ],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 8000,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**']
    },
    proxy: {
      '/api': { target: 'http://127.0.0.1:9993', changeOrigin: true }
    }
  },
  test: {
    root: './src',
    include: ['**/**/*.{test,spec}.?(c|m)[jt]s?(x)'],
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./utils/testUtils/setupTest.tsx'],
    coverage: {
      exclude: ['**/__mocks__/**', '**/typings/**', '**/i18n/**', '**/services/**', '**/*.d.ts']
    }
  },
}))
