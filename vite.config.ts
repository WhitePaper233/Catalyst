import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // Tauri expects a fixed port, fail if that port is not available
  server: {
    strictPort: true,
  },
  // to make use of `TAURI_PLATFORM`, `TAURI_ARCH`, `TAURI_FAMILY`,
  // `TAURI_PLATFORM_VERSION`, `TAURI_PLATFORM_TYPE` and `TAURI_DEBUG`
  // env variables
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Tauri uses Chromium on Windows and WebKit on macOS and Linux
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // 为调试构建生成源代码映射 (sourcemap)
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  // less定制主题
  css: {
    preprocessorOptions: {
      less: {
        modifyVars: {
          'body-background': '#282c34',
          'font-family': 'Inter, Avenir, Helvetica, Arial, sans-serif',
          'font-size-base': '14px',
          'font-size-sm': '12px',
          'line-height-base': '24px',
          'popover-background': '#21252b',
          'box-shadow-base': '0 2px 8px rgba(0, 0, 0, 0.15)',
          'border-radius-base': '0px',
          'menu-item-vertical-margin': '0px',
        },
        javascriptEnabled: true,
      },
    },
  },
})
