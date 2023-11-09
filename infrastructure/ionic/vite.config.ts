import legacy from "@vitejs/plugin-legacy"
import react from "@vitejs/plugin-react"
import { defineConfig } from "vite"

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), legacy()],
  // test: {
  //   environment: "jsdom",
  //   globals: true,
  //   setupFiles: "./src/setupTests.ts",
  // },
})
