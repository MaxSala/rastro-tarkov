import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import pkg from "./package.json";

// Puerto del backend en desarrollo (cargo run -- --dev). Debe coincidir con config.json.
const BACKEND = "http://127.0.0.1:47821";

export default defineConfig({
  plugins: [react(), tailwindcss()],
  define: { __APP_VERSION__: JSON.stringify(pkg.version) },
  build: {
    // es2022: los textos del idioma se cargan con await de nivel superior (WebView2 y WebKitGTK lo soportan).
    target: "es2022",
    outDir: "dist",
    emptyOutDir: true,
    // Nada inline: la CSP del backend solo permite scripts, estilos y fuentes de 'self'.
    assetsInlineLimit: 0,
    modulePreload: { polyfill: false },
    sourcemap: false,
    // App local: un único bundle (React + Leaflet + zod) es aceptable.
    chunkSizeWarningLimit: 800,
  },
  server: {
    port: 5173,
    strictPort: true,
    host: "127.0.0.1",
    proxy: {
      "/api": { target: BACKEND, changeOrigin: true },
      "/docs": { target: BACKEND, changeOrigin: true },
      "/ws": { target: BACKEND, changeOrigin: true, ws: true },
    },
  },
});
