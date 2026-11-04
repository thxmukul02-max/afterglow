# Afterglow for Windows

Frosted-glass media player, packaged as a small Windows installer with Tauri.
The app itself is `src/index.html`. Edit it, rebuild, and the installer updates.

## Option A: build online (nothing to install)
1. Create a free GitHub account and a new empty repository.
2. Upload this whole folder (keep the `.github` folder). GitHub Desktop or `git push` is easiest.
3. Open the repo's **Actions** tab, choose **Build Windows installer**, click **Run workflow**.
4. After about 8 minutes, open the finished run and download **Afterglow-Windows-Installer**. Unzip it and run the `setup.exe` inside.

## Option B: build on your own PC
Install once: Node.js (LTS), Rust (rustup.rs), and "Desktop development with C++" from Visual Studio Build Tools.
Then, inside this folder:

    npm install
    npm run tauri build

Your installer will be in `src-tauri/target/release/bundle/nsis/`.

## Notes
- First build downloads a lot and is slow. Later builds are much faster.
- Windows SmartScreen may warn about an unknown publisher because the installer is not code-signed. Choose "More info" then "Run anyway".
- Installs per user (no admin rights needed). WebView2 is installed automatically if missing.
