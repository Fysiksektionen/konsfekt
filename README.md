## About

The project's backend is a Rust application built with the [Actix Web](https://actix.rs) framework.
Its frontend is a static site generated using [SvelteKit](https://svelte.dev), 
with UI components drawn from the [shadcn-svelte](https://www.shadcn-svelte.com/docs/components) library.
Authentication relies on session handling modeled after the principles laid out by [lucia-auth](https://lucia-auth.com), 
with login handled via Google.

## Develop
### Dependencies
- rust
- npm

Make sure to run `npm install` when inside the `frontend` directory to download all frontend dependencies.

### Setup Google

Get Google OAuth client credentials from [Google](https://console.developers.google.com/). You will need a client id and a client secret.
Make sure to add `http://127.0.0.1:8080` as an Authorized JavaScript origin and `http://127.0.0.1:8080/api/auth/google/callback`
as an Authorized redirect URI.

> Due to a limited set of authorized origins Google allows, simple developing is limited to localhost 
unless using a tunneling service.

Create a `.env` from the `template.env` file with the following fields filled in
- `GOOGLE_CLIENT_ID`
- `GOOGLE_CLIENT_SECRET`

### Setup Swish
To test the functionality of Swish, we recommend using the [Swish Sandbox](https://developer.swish.nu/documentation/environments).
It will enable you to test making payments end-to-end with the Swish test app for [Android](https://appdistribution.firebase.google.com/i/6e190185a34cb2f3) and [iOS](https://testflight.apple.com/join/iQTsRg5b).

1. Get a hold the Swish Sandbox certificates from a developer at Fysiksektionen.
2. Extract the provided zip-archive into `certificates/sandbox/`.
3. Set the environment variable `SWISH_NUMBER` to the merchant number found in `certificates/sandbox/details.txt`.
4. Setup the Swish test app with one of the listed user numbers in `details.txt`.
5. Create a [test BankID](https://developers.bankid.com/test-portal/bankid-for-test).
6. Make sure `SWISH_ENVIRONMENT` is set to `sandbox`

> NOTE: `https` is required when testing Swish functionality. See [Tunneling](#tunneling---develop-with-https).

See [docs/swish-production.md](docs/swish-production.md) for what's left to go live with Swish production.

### Running
Depending on what you want to develop, the app can be run in different ways.

The two main flags determining how the app is run is `--local` and `--static`.

#### Auto-reloaded frontend
For example, if you want to run it locally with a live-preview of the frontend you'll do:

```
cargo run --bin konsfekt -- --local
```
and
```
npm run dev
```
from within the frontend directory. This will start the app in two processes.

> NOTE: Middleware will not be applied to frontend routes (/) when serving the frontend separately. This means that some redirects and permission checks won't work.

> If you need a valid session to do something, please go to `/login` manually and log in. 

> E.g `/admin` will be accessible with a normal account.

#### Simple Local setup
If you want all redirects and permission checks to work, you'll need to run the app with a static frontend.
This can be done by passing both the `--local` and `--static`. A prerequisites to this is building 
the frontend with `npm run build` from within the frontend directory.

```
cargo run --bin konsfekt -- --local --static
```

This will serve the frontend on the backend route `/`.

> NOTE: You will not have a secure connection (no https). So you wont be able to test out Swish functionality.

#### Tunneling - Develop with HTTPS 
In order to test stuff like Swish you need the app running with a secure HTTP connection. 
You can accomplish this by using a tunneling service such as serveo.net

1. Create an account at serveo to get a persistent domain.
2. Add that domain as the `SITE_DOMAIN` env variable.
3. Setup that domain in the Google OAuth Client
4. Build the frontend
5. Run the backend with no flags.
6. ssh into the tunneling service and forward `0.0.0.0:8080` to Serveo's persistent domain

Example:
```bash
$ ssh -R konsfekt:80:0.0.0.0:8080 serveo.net
Forwarding HTTP traffic from https://konsfekt.serveousercontent.com
```

### Tauri (Mobile App) !MAY BUILD NATIVE APPS!
This project uses [Tauri](https://v2.tauri.app) to serve the web page as a mobile app. 
To get started first ensure you have all the [prerequisites](https://v2.tauri.app/start/prerequisites/) setup.

To run the app as a dev server use `npx tauri dev`.

Before building the app you need to create the file `.env.tauri` inside the `frontend` directory. Set the variable `VITE_API_URL` to the same value as `SITE_DOMAIN` in `.env`.

Build the app with `npx tauri build`

## Docker
Dependencies:
- docker (docker compose)

Create a `.env` from `template.env` and set a value for the following:
- `SITE_DOMAIN` domain the webapp should be accessible at (used by Caddy to request its TLS certificate)
- `DATABASE_DIR` host path to store the database and uploaded images
- `PERMISSION_TABLE_PATH` host path to `permission_table.json`
- `CERTIFICATES_DIR` host path to the Swish certificates (see [Setup Swish](#setup-swish))
- `SWISH_NUMBER` the merchant Swish number
- `SWISH_ENVIRONMENT` (`prod` or `sandbox`)
- `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET` see [Setup Google](#setup-google)

Run the containers with `docker compose up --build` (`--build` flag only needed the first time, or when new code/migrations has landed).

This starts two containers:
- `konsfekt` — the app itself, built from the root `Dockerfile`. Not reachable directly from the host; only `caddy` talks to it, over the internal Docker network on port `8080`.
- `caddy` — reverse proxy that terminates HTTPS for `SITE_DOMAIN` and forwards everything to `konsfekt`. 
Publishes ports `80`/`443` on the host and automatically obtains a Let's Encrypt certificate via ACME (`http-01`/`tls-alpn-01`), 
which requires that `SITE_DOMAIN` actually resolves to this host and that ports 80/443 are reachable from the internet — see [docs/port-forwarding.md](docs/port-forwarding.md) if deploying behind a home router.
