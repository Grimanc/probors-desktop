# Probors Desktop

This repository wraps the existing Next.js app (../probors-front/probors-react-2.0) into a Tauri desktop app for macOS and Windows.

Key points:
- Runs the Next.js server as a child process in production so API routes and SSR remain available.
- Keeps app code in the original repo (the desktop repo expects the Next app at `../probors-front/probors-react-2.0`).
- CI is included to build macOS and Windows installers (recommended).

Prerequisites:
- Rust toolchain (stable) and cargo.
- Node.js (for building the Next app during CI and on developer machines).
- For local development, run `npm run dev` from this repo.

Quick commands:
- Dev: `npm run dev` (starts Next dev server and Tauri dev)
- Build: `npm run build` (builds Next app, then runs `tauri build` to produce installers)
 - Start Next only: `npm run start:next`

Production notes:
- By default the desktop app is configured to load the remote site (https://probors.com). This makes the desktop app lightweight and uses the live production backend.
 - By default the desktop app is configured to load the remote dashboard (https://dashboard.probors.com). This makes the desktop app lightweight and uses the live production backend.
- To run the app against a local Next server (development), set the environment variable `PROBORS_LOCAL_NEXT=true` when running the Tauri app (this will spawn `npm run start` in the sibling frontend repo).

Example (development):

- Start dev (Next + Tauri): `npm run dev`
- Or run Tauri with local server spawn: `PROBORS_LOCAL_NEXT=true npx tauri dev`

Release notes:
- CI workflow `/.github/workflows/desktop-release.yml` will build macOS and Windows bundles and upload them as artifacts when pushing to `main`.

**Icon, auto-updates & landing page download:**  
See **[DESKTOP_UPDATES_AND_DOWNLOAD.md](./DESKTOP_UPDATES_AND_DOWNLOAD.md)** for:
- Using the black PB icon (already set from `probors-front/.../probors-icon-black.png`)
- Enabling in-app auto-updates (signing keys, `latest.json`, endpoints)
- Adding a “Download” link on your landing page (GitHub Releases or your own URL)

**Release on GitHub and test updates:** See **[RELEASE_AND_UPDATE_TEST.md](./RELEASE_AND_UPDATE_TEST.md)** for setting your GitHub org/repo, pushing to main to create a release, and testing the in-app updater.

See `src-tauri` for Tauri-specific code and configuration.

Secrets for release & signing
- To enable automatic GitHub Releases and artifact upload, ensure `GITHUB_TOKEN` is available to Actions (it is provided automatically). For advanced release scripting you may add a personal token as `secrets.RELEASE_TOKEN`.
- macOS code signing / notarization (optional). Add these secrets to your repo:
  - APPLE_ID (Apple developer email)
  - APPLE_APP_PASSWORD (app-specific password for notarization)
  - MACOS_SIGNING_IDENTITY (e.g., "Developer ID Application: Your Company (TEAMID)")
  - APPLE_TEAM_ID
- Windows code signing (optional). Add:
  - WINDOWS_PFX (base64-encoded PFX file)
  - WINDOWS_PFX_PASSWORD

If signing secrets are present CI will attempt to sign and notarize artifacts. If not, CI still builds and uploads unsigned artifacts.

