import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
	plugins: [
        sveltekit(),
        tailwindcss()
    ],
    server: {
        host: "127.0.0.1",
        proxy: {
            "/api": {
                target: "http://127.0.0.1:8080",
                changeOrigin: true,
                secure: false
            },
            "/uploads": {
                target: "http://127.0.0.1:8080",
                changeOrigin: true,
                secure: false
            }
        }
    }
});
