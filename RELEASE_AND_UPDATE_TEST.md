# Release on GitHub and Test Updates

**Important:** GitHub Actions only runs workflows at the **root of the repo**. If your **Git repo is the `probors-desktop` folder** (this folder is the repo root), then the workflow lives here: **`probors-desktop/.github/workflows/desktop-release.yml`** — that is the repo root, so GitHub will run it when you push. All paths in the workflow are relative to that root (e.g. `src-tauri/`, no `probors-desktop/` prefix).

## 1. Set your GitHub repo in the app

The updater checks GitHub for `latest.json`. Set your repo in **`src-tauri/tauri.conf.json`**:

1. Open `src-tauri/tauri.conf.json`.
2. Find `plugins.updater.endpoints`.
3. Replace `OWNER` and `REPO` with your GitHub org and repo name.

Example: if your repo is `https://github.com/mycompany/probors-react-2.0`, use:

```json
"endpoints": [
  "https://github.com/mycompany/probors-react-2.0/releases/latest/download/latest.json"
]
```

Save the file.

## 2. (Optional) Sign updates

For the in-app updater to install new versions, builds must be signed. If you skip this, the release will still be created and users can download from GitHub; the app just won’t offer in-app updates.

- Generate keys: `cd probors-desktop && npx tauri signer generate -w ~/.tauri/probors.key`
- Add **TAURI_SIGNING_PRIVATE_KEY** to GitHub Secrets (repo → Settings → Secrets and variables → Actions): paste the **contents** of `~/.tauri/probors.key`.
- Put the **public** key in `src-tauri/tauri.conf.json` under `plugins.updater.pubkey` (you may have done this already).

Without this secret, the workflow still runs and uploads installers + `latest.json`, but `.sig` files will be missing so the updater will reject the update. Users can still download new versions from the GitHub release page.

## 3. Push to main to create a release

1. Commit and push your changes to the **main** branch (including the `tauri.conf.json` endpoint change).
2. The workflow **Build desktop releases** runs on push to main.
3. In GitHub: **Actions** → select the run → wait for **build** and **release** to finish.
4. Open **Releases** → you should see a new release (e.g. **ProBors Desktop v0.1.0**) with:
   - macOS and Windows installers (from `artifacts/macos` and `artifacts/windows`, including subdirs like `dmg/`, `macos/`).
   - **latest.json** (used by the updater).

Download link for users:  
`https://github.com/YOUR_ORG/YOUR_REPO/releases/latest`

## 4. Test that updates work

### 4a. Install the current release

1. From the release page, download and install the app (e.g. DMG on Mac).
2. Open the app and leave it running (or close and reopen once).

### 4b. Publish a “new” version

1. Bump the version in **`src-tauri/tauri.conf.json`** (e.g. change `"version": "0.1.0"` to `"0.1.1"`).
2. Commit and push to **main**.
3. Wait for the workflow to finish and create a new release with the new version.

### 4c. Trigger the update in the app

1. Open the installed app (still 0.1.0).
2. The updater checks **releases/latest/download/latest.json** on startup. If a newer version exists (0.1.1), the app should prompt to download and install the update.
3. If you see the prompt, accept; the new build should download and install (and the app may restart).
4. If you **did not** add **TAURI_SIGNING_PRIVATE_KEY**, the update may be rejected (signature missing). In that case, either add the key and re-run the workflow, or confirm that the release page still shows the new installers and that manual download works.

### Quick checklist

- [ ] Replaced `OWNER`/`REPO` in `tauri.conf.json` with your GitHub org/repo.
- [ ] Pushed to main and confirmed a release with installers + `latest.json`.
- [ ] (Optional) Added `TAURI_SIGNING_PRIVATE_KEY` and have a public key in `tauri.conf.json`.
- [ ] Installed the app from the release, then bumped version, pushed, and saw the update prompt (or confirmed manual download from the new release).
