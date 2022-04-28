import vueI18n from '@intlify/vite-plugin-vue-i18n'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'
import ElementPlus from 'unplugin-element-plus/vite'
import { defineConfig } from 'vite'

export default defineConfig({
  plugins: [vue(), ElementPlus(), vueI18n({ include: resolve(__dirname, 'src/i18n/locales/**') })],
  server: {
    port: 8000,
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
    },
  },
})
