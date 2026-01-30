# ProBors Desktop: Auto-Updates & Landing Page Download

## Icon

The app icon is set from `probors-front/probors-react-2.0/public/icons/probors-icon-black.png`, copied to `src-tauri/icons/icon.png`. To change it, replace that file (use a square PNG; RGBA is preferred).

---

## Auto-updates (in-app)

The app is configured to check for updates using the Tauri updater plugin. To make updates work end-to-end:

**Before updates work:** Replace the placeholder in `src-tauri/tauri.conf.json`: set `plugins.updater.pubkey` to your real public key and `plugins.updater.endpoints` to your `latest.json` URL. Until then, the app will run normally but update checks will fail (no prompt).

#### How do I make the endpoint?

The “endpoint” is just the **URL** where the file `latest.json` is served. You don’t run a separate server — you put `latest.json` somewhere that’s publicly reachable and use that URL.

**Option 1 – S3 (recommended if you already use AWS)**  
Your repo’s workflow **publish-to-s3** already uploads `latest.json` to S3 when you have these GitHub Secrets set:

- `AWS_ACCESS_KEY_ID`
- `AWS_SECRET_ACCESS_KEY`
- `S3_BUCKET` (bucket name)
- `S3_PUBLIC_HOST` (the host used to reach the bucket, e.g. `your-bucket.s3.us-east-1.amazonaws.com` or a CloudFront domain like `d1234abcd.cloudfront.net`)

Then the endpoint URL is:

`https://<S3_PUBLIC_HOST>/probors-desktop/latest.json`

So you “make” the endpoint by:

1. Creating an S3 bucket (if you don’t have one).
2. Making the bucket (or the `probors-desktop/` prefix) publicly readable, or putting CloudFront in front and using that domain as `S3_PUBLIC_HOST`.
3. Adding the four secrets above in your repo (Settings → Secrets and variables → Actions).
4. Pushing to `main` so the workflow runs; it uploads `latest.json` to `s3://<bucket>/probors-desktop/latest.json`.
5. Putting that URL in `tauri.conf.json` as the only entry in `plugins.updater.endpoints` (see “How to set the endpoint” below).

**Option 2 – GitHub Releases**  
You “make” the endpoint by creating a release and uploading a file named `latest.json` as an asset. The URL is then:

`https://github.com/<org>/<repo>/releases/latest/download/latest.json`

You can use [Tauri Action](https://github.com/tauri-apps/tauri-action) to build and upload `latest.json` automatically, or generate it yourself and add it to each release.

---

#### How to set the endpoint (step-by-step)

1. Open **`probors-desktop/src-tauri/tauri.conf.json`** in your editor.
2. Find the **`"plugins"`** → **`"updater"`** → **`"endpoints"`** array (one line with a URL in quotes).
3. Replace the URL with your real endpoint:

   - **If you use S3 (or your own CDN):**  
     Use the URL where `latest.json` is served, e.g.  
     `https://YOUR_PUBLIC_HOST/probors-desktop/latest.json`  
     Replace `YOUR_PUBLIC_HOST` with the same value you use in GitHub Secrets as `S3_PUBLIC_HOST` (e.g. `d1234abcd.cloudfront.net` or `your-bucket.s3.amazonaws.com`).  
     Example:  
     `"endpoints": [ "https://d1234abcd.cloudfront.net/probors-desktop/latest.json" ]`

   - **If you use GitHub Releases:**  
     Use:  
     `"endpoints": [ "https://github.com/YOUR_ORG/YOUR_REPO/releases/latest/download/latest.json" ]`  
     Replace `YOUR_ORG` and `YOUR_REPO` with your GitHub org and repo name.

4. Save the file. The app will use this URL to check for updates.

### 1. Generate signing keys (one-time)

From the repo root:

```bash
cd probors-desktop
npx tauri signer generate -w ~/.tauri/probors.key
```

- **Public key**: Copy the **public** key and put it in `src-tauri/tauri.conf.json` under `plugins.updater.pubkey` (replace `REPLACE_WITH_PUBLIC_KEY_FROM_tauri_signer_generate`).
- **Private key**: Keep `~/.tauri/probors.key` (or your chosen path) secret and backed up. You need it for every release build.

### 2. Set the update endpoint

**Where to edit:** Open `probors-desktop/src-tauri/tauri.conf.json` and find the `"plugins"` → `"updater"` → `"endpoints"` array. Replace the placeholder URL with your real URL (see below).

#### Option A – GitHub Releases

1. Use the [Tauri Action](https://github.com/tauri-apps/tauri-action) in CI to build, sign, and upload a `latest.json` asset to each release (or create/upload it yourself).
2. In `tauri.conf.json` set:

```json
"endpoints": [
  "https://github.com/YOUR_ORG/YOUR_REPO/releases/latest/download/latest.json"
]
```

Replace `YOUR_ORG` and `YOUR_REPO` with your GitHub org and repo (e.g. `mycompany` and `probors-react-2.0`).

#### Option B – Your own server / S3

1. **Set the endpoint in `tauri.conf.json`** to the URL where you serve `latest.json`:

```json
"endpoints": [
  "https://YOUR_PUBLIC_HOST/probors-desktop/latest.json"
]
```

Replace `YOUR_PUBLIC_HOST` with your actual host, for example:
- S3 + CloudFront: `d1234567890.cloudfront.net` or your custom domain
- Your own server: `download.probors.com` or whatever serves the file

So the full URL might be `https://d1234567890.cloudfront.net/probors-desktop/latest.json`.

2. **Serve a `latest.json` file** at that URL in Tauri’s format. The file must be valid JSON and include at least `version`, `platforms`, and for each platform `url` and `signature`. Example:

```json
{
  "version": "0.1.0",
  "notes": "Release notes (optional)",
  "pub_date": "2025-01-29T12:00:00Z",
  "platforms": {
    "darwin-aarch64": {
      "signature": "CONTENTS_OF_PROBORS_0.1.0_AARCH64_DMG_SIG_FILE",
      "url": "https://your-host.com/probors-desktop/abc123/ProBors_0.1.0_aarch64.dmg"
    },
    "darwin-x86_64": {
      "signature": "CONTENTS_OF_X86_64_SIG",
      "url": "https://your-host.com/.../ProBors_0.1.0_x86_64.dmg"
    },
    "windows-x86_64": {
      "signature": "CONTENTS_OF_NSIS_OR_MSI_SIG",
      "url": "https://your-host.com/.../ProBors_0.1.0_x64-setup.nsis.zip"
    }
  }
}
```

- **`signature`**: Copy the **entire contents** of the `.sig` file Tauri generates next to each installer (e.g. `ProBors_0.1.0_aarch64.dmg.sig`). Paste that string as the value of `signature` (no file path, no URL).
- **`url`**: Public URL to download that platform’s installer (or updater bundle, e.g. `.tar.gz` on macOS, `.nsis.zip` on Windows if you use v1 compatible format).

The repo’s GitHub Action **publish-to-s3** can generate and upload this `latest.json` for you if you set `S3_BUCKET` and `S3_PUBLIC_HOST`; see the workflow and the “Create latest metadata JSON” step (it is updated to output Tauri’s format when signatures are available).

### 3. Build with the private key

When building installers that should be updateable, set the signer private key (and optional password):

```bash
# Mac/Linux
export TAURI_SIGNING_PRIVATE_KEY="$(cat ~/.tauri/probors.key)"
# optional:
export TAURI_SIGNING_PRIVATE_KEY_PASSWORD=""

cd probors-desktop
npm run build
```

On Windows (PowerShell):

```powershell
$env:TAURI_SIGNING_PRIVATE_KEY = Get-Content -Path "$env:USERPROFILE\.tauri\probors.key" -Raw
npm run build
```

Tauri will produce the installers and the `.sig` files used by the updater.

### 4. Publish each release

- **GitHub**: Create a release and upload the installers plus the **single** `latest.json` that covers all platforms (Tauri Action does this for you if you use it).
- **S3 / custom**: Upload the installers and the `latest.json` (and any `.sig` content you embed in it) so the URL in `endpoints` returns that JSON.

After that, the desktop app will check the endpoint on startup and, if a newer version is available, can prompt the user to download and install the update.

---

## Landing page download

You want a “Download” button on your landing page that always points to the latest desktop build.

### Start the download from your website (recommended)

So users don’t have to open GitHub: your site can link to **your domain**, and your server redirects to the latest GitHub asset so the file download starts immediately.

**Already set up:** The Next app has an API route that does this:

- **Download for Mac:**  
  `https://dashboard.probors.com/api/download-desktop?platform=mac`  
  (or `https://probors.com/api/download-desktop?platform=mac` if your landing page is on the same app)
- **Download for Windows:**  
  `https://dashboard.probors.com/api/download-desktop?platform=windows`

When the user clicks, they stay on your domain for the click; the server redirects to the latest GitHub release asset and the browser starts the download. No GitHub page in between.

**What you need to do:**

1. Set the GitHub repo in your environment (e.g. in Vercel / your host):
   ```bash
   GITHUB_RELEASE_REPO=your-org/your-repo
   ```
   Example: `GITHUB_RELEASE_REPO=mycompany/probors-react-2.0`

2. On your landing page, use these links (adjust the host if your site is `probors.com`):
   ```html
   <a href="https://dashboard.probors.com/api/download-desktop?platform=mac">Download for Mac</a>
   <a href="https://dashboard.probors.com/api/download-desktop?platform=windows">Download for Windows</a>
   ```

The route picks the right asset from the latest GitHub release (e.g. `.dmg` for Mac, `.msi` or `.exe` for Windows) and redirects to it. No code change needed when you publish a new release.

---

### Option A – Link to GitHub Releases page

If you prefer to send users to GitHub:

1. **Latest release page**  
   Link:  
   `https://github.com/YOUR_ORG/YOUR_REPO/releases/latest`  
   Users land on the release page and pick macOS (e.g. DMG) or Windows (e.g. MSI/NSIS).

### Option B – Your own host (e.g. S3 + CloudFront)

If you host installers and `latest.json` on S3 (or similar):

1. **Stable download URL**  
   e.g.  
   `https://download.probors.com/desktop/latest/mac`  
   (or `/windows`) that you update to point to the current DMG/EXE (e.g. redirect or rewrite rule).

2. **Landing page**  
   - “Download for Mac” → `https://download.probors.com/desktop/latest/mac`  
   - “Download for Windows” → `https://download.probors.com/desktop/latest/windows`  

You can keep using the same URL; only the file it serves (or redirects to) changes with each release.

### Example landing page snippet

```html
<a href="https://github.com/YOUR_ORG/YOUR_REPO/releases/latest" target="_blank" rel="noopener">
  Download for Mac / Windows
</a>
```

Or two buttons:

```html
<a href="https://github.com/YOUR_ORG/YOUR_REPO/releases/latest">Download for Mac</a>
<a href="https://github.com/YOUR_ORG/YOUR_REPO/releases/latest">Download for Windows</a>
```

(Both can go to the same “latest” page; users pick the right file.)

---

## Summary

| Goal                         | Action |
|-----------------------------|--------|
| Use black PB icon           | Done; source: `probors-icon-black.png` → `src-tauri/icons/icon.png`. |
| In-app auto-updates         | 1) Generate keys, 2) Set `pubkey` and `endpoints` in `tauri.conf.json`, 3) Build with `TAURI_SIGNING_PRIVATE_KEY`, 4) Publish installers + `latest.json` (e.g. via Tauri Action or S3). |
| Download on landing page    | Link “Download” to your latest release (e.g. GitHub `.../releases/latest`) or to your own stable URL that serves the latest Mac/Windows installer. |
