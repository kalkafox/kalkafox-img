import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          react: ['react', 'react-dom'],
          'react-spring': ['@react-spring/web'],
          three: ['three'],
          'three-extras': ['@react-three/fiber'],
        },
      },
    },
  },
})
