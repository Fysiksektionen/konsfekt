### Dev (any platform)
- Vite proxy forwards /api/... to 127.0.0.1:8080
- Origin is http://127.0.0.1:5173 → matched by frontend_url in CORS
- VITE_API_URL is empty → apiUrl() returns relative URLs, proxy handles routing

### Prod - browser (release build)
- static_frontend forced true → backend serves frontend/build
- vite build uses .env.production (no VITE_API_URL set)
- apiUrl() returns relative URLs → same origin as backend, no CORS

### Prod - Tauri app
- tauri build runs vite build --mode tauri → uses .env.tauri
- VITE_API_URL=https://f.kth.se/konsfekt baked into bundle
- apiUrl() returns full URLs → calls https://f.kth.se/konsfekt/api/...
- Origin is tauri://localhost → backend CORS must allow it
