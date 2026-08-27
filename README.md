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

The app ships as a single container image, `ghcr.io/fysiksektionen/konsfekt`, which
serves both the API and the static frontend on port `8080`. It does **not**
terminate HTTPS — the host is expected to run its own reverse proxy that forwards
`SITE_DOMAIN` traffic to that port.

`docker-compose.yml` is the reference for running it: what env vars and host paths
a deployment needs. It pulls the published image; it does not build.

### Releasing an image (developers)

[`.github/workflows/docker.yml`](.github/workflows/docker.yml) builds from the root
`Dockerfile` and pushes to GHCR on any `v*` tag. To cut a release:

1. Make sure `main` is green and contains everything you want in the release.
2. Tag with a [semver](https://semver.org) version and push the tag:
   ```
   git tag v1.4.0
   git push origin v1.4.0
   ```
3. The workflow publishes `ghcr.io/fysiksektionen/konsfekt` as `1.4.0`, `1.4`, and
   `latest`.
4. First release only: an org admin must set the GHCR package's visibility to
   public (Fysiksektionen → Packages → `konsfekt` → Package settings), otherwise
   the host needs a personal access token with `read:packages` to pull.

### Deploying on the host

Needs `docker` with the compose plugin, and a copy of `docker-compose.yml` (plus
this repo's `template.env` for reference).

1. Create `.env` from `template.env` and set:
   - `SITE_DOMAIN` public URL the webapp is served at (used for OAuth redirects and cookies)
   - `DATABASE_DIR` host path to store the database and uploaded images
   - `PERMISSION_TABLE_PATH` host path to `permission_table.json`
   - `CERTIFICATES_DIR` host path to the Swish certificates (see [Setup Swish](#setup-swish))
   - `SWISH_NUMBER` the merchant Swish number
   - `SWISH_ENVIRONMENT` (`prod` or `sandbox`)
   - `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET` see [Setup Google](#setup-google)
2. Create the paths referenced above: the `DATABASE_DIR` and `CERTIFICATES_DIR`
   directories, and a `permission_table.json` file (start from the one in this repo).
3. Point the host's reverse proxy at `127.0.0.1:8080` and let it serve HTTPS for
   `SITE_DOMAIN`.
4. Start it:
   ```
   docker compose pull
   docker compose up -d
   ```
5. To upgrade later, repeat step 4 — `pull` fetches the new `latest`, `up -d`
   recreates the container. Pin a specific release by setting
   `image: ghcr.io/fysiksektionen/konsfekt:1.4.0` in `docker-compose.yml` instead.

The permission table is bind-mounted and can be edited on the host, but the app
reads it only at startup — run `docker compose restart konsfekt` after changing it.

### Local Docker
To build from source and run the app locally (local mode, no HTTPS):

```
docker compose -f docker-compose.yml -f docker-compose.local.yml up --build konsfekt
```

`docker-compose.local.yml` overrides the image with a local build (`konsfekt:local`)
so this never runs a pulled release. Swish operations will not work without HTTPS.
