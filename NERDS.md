# NERDS edition: what changed and what's next

This repository is ArtCraft Launcher reskinned for NERDS. It is a modified version of
[storytold/craft-launcher](https://github.com/storytold/craft-launcher) (MIT OR Apache-2.0); this
file lists the changes, as the Apache License asks of modified files.

## Stage 1: the launcher (this change)

**Following upstream**: this branch merges upstream's `main` (last: ArtCraft Launcher 0.10.0,
which installs the apps' dual-scope Windows installers for one person or for everyone, in the
scope they were installed in). Upstream's community app, SolidCraft, has no NERDS build and is
left out of the suite (`catalog::SOLIDCRAFT` stays only as a test example). The NERDS Launcher
keeps its own self-update (`native/src/self_update.rs`; upstream moved its own into
`craft-update`, which would look for ArtCraft's signed releases) and its own install folder,
`%LOCALAPPDATA%\Programs\NERDS Launcher` (`packaging/windows/install-policy.json`).

**Look**
- Colours: `crates/ui-egui/src/theme.rs`. NERDS blue `#0077C0` (sampled from the NERDS logo) for
  accents and primary buttons; deep navy `#0D1824` surfaces in dark mode, cool white `#F5F7FA` in
  light mode. Status colours (green, amber, red) are unchanged. The web build's loading page
  (`apps/artcraft-launcher-web/index.html`) matches.
- Header logo: the NERDS N mark and N.E.R.D.S. side by side (`docs/brand/nerds-logo-horizontal*.svg`).
- App icon, tray icon, Windows installer art and the macOS DMG background: NERDS.
- App icons: one tile per app in its Craft app's colour, with a white symbol for what it does
  (`assets/app-icons/source/*.svg`, drawn by `docs/brand/source/make_icons.py`, rendered by
  `packaging/icons.sh`). Each app's accent in the launcher is its Craft colour too.

**Names**
- NERDS Launcher, the NerdsCraft Suite, and each app as "NERDS" + what it does
  (`crates/core/src/catalog.rs`). The NERDS builds of the apps install under these names too, so
  `name` (what install and detection match) and `display` are the same. The upstream name is the
  first of `former_names`: earlier NERDS builds installed under it, and the apps' data folders,
  Windows registry key and Mac `<Name>Version` key keep it.
- All user-facing text, in all eleven languages, and the packaging (MSI, DMG, .deb/.rpm, Flatpak)
  say NERDS Launcher. The Windows installer has its own upgrade code and component IDs, so it
  never replaces or clashes with an ArtCraft Launcher install.

**Home and updates**
- The launcher's repository is github.com/nerds-coach/nerds-launcher (`LAUNCHER_REPO`): it looks
  for its own new versions there, never at the upstream ArtCraft Launcher.

**Publisher and files**
- Publisher: NERDS Craft (Windows exe and installer, registry key `HKCU\Software\NERDS Craft\NERDS
  Launcher`, Linux package metadata). App detection accepts the apps' NERDS Craft and their
  earlier Learning Machines LLC (`detect.rs`, `PUBLISHERS`).
- Release files are named `nerds-launcher-<version>-…` (`catalog::LAUNCHER_ASSET_STEM`); self-update
  looks for those first. In the Windows zip the program is `NERDS Launcher.exe`; elsewhere the
  binary and cargo packages keep the upstream `artcraft-launcher` name.
- Each app's Website link is left out while it would only repeat its GitHub link
  (`CraftApp::has_own_website`).
- The macOS DMG window (`packaging/macos/dmg/background.svg`) says NERDS Launcher; its text is
  outlined with `docs/brand/source/outline_text.py`, and the release workflow redraws
  `background.tiff` and `dmg-layout.DS_Store` from it.

**Building and releasing** (`.github/workflows/`)
- `release.yml` builds the Windows installer and portable zip and the macOS universal DMG on
  GitHub's free runners, writes `SHA256SUMS.txt` and publishes the GitHub Release `v<version>`.
  Run it from the Actions tab (Release → Run workflow) after bumping the version in `Cargo.toml`
  (`cargo xtask version set X.Y.Z`). Signing secrets are optional (listed at the top of the file).
- `ci.yml` runs fmt, clippy and the tests on Linux, Windows and macOS, and draws screenshots of
  the launcher. Each job publishes its logs (and the screenshots) to a `ci-output/<job>` branch,
  and the release publishes the redrawn DMG window files to `ci-output/dmg-layout`, so they can
  be read with plain git (`git fetch origin 'refs/heads/ci-output/*:refs/remotes/origin/ci-output/*'`).
- Removed: the upstream FreeBSD, Nix, SonarCloud and Windows-on-Arm workflows (they need
  ArtCraft's paid runners or accounts). Linux and FreeBSD packaging scripts are kept, unused.
- Unsigned builds: Windows asks "More info → Run anyway" and macOS "Privacy & Security → Open
  Anyway" on first launch. On Windows an unsigned launcher can't install its own updates (it
  checks the publisher's signature); it still says when a new version is out, and links to it.

**Switched off until NERDS has its own**
- Website, Discord and social links: empty in `catalog.rs`; an empty link is hidden in the
  footer, Settings and the tray menu. Fill them in to show them. (GitHub: github.com/nerds-coach.)
- The upstream release feed's announcements and alerts (`UPSTREAM_NOTICES`): off, as they are
  ArtCraft's news. Release information from the feed is still used.

**The apps: NERDS builds** (`nerds-coach/nerds-photo` … `nerds-grid`)
- Each app is installed from its own NERDS repository (`CraftApp::repo`). That repository holds no
  app code: its build workflow takes the latest official release of the upstream app, lays the
  NERDS icon over the app's `assets/app-icon/` (every file, same names and sizes, made by
  `docs/brand/source/make_icon_set.py`), gives it its NERDS names, builds Windows x64 (MSI,
  portable zip) and macOS universal (DMG) on free runners, and publishes them with the upstream
  tag, file names and release notes. It checks for a new upstream release every six hours.
- So an app shows its NERDS icon everywhere (taskbar, Start menu, Dock, window, in-app mark) and
  its NERDS name everywhere people read it: title bar, menus, About, messages and translations,
  the Windows installer, Start menu and Apps list, the Mac app's name in Finder and the Dock, and
  its disk image. The renaming (`.github/nerds-rename.py` in each app repository) changes words
  people read and leaves identifiers alone (data folders, file formats, registry keys, bundle
  ids), so settings and documents carry over. The apps that show ArtCraft's logo show NERDS's.
  Their Help menus still link to ArtCraft's website and Discord, the community behind the apps.
- Each app's own update check (the shared `craft-update` library, `craft_update::suite::<APP>`)
  looks at its NERDS repository and names the app by its NERDS name, so it never offers
  ArtCraft's build of the app.
- A Mac app installed under its upstream name (`PhotoCraft.app`) is replaced by the NERDS-named
  bundle on its next update or reinstall; the old copy goes to the Trash (`install::macos`).
- The ArtCraft release feed is off (`USE_RELEASE_FEED`): it only knows the upstream builds.
- NERDS builds are ad-hoc signed on macOS: the launcher accepts a Mac app, or its own update,
  when its signature is intact (`APP_SIGNING_TEAM` is `None`; `install::macos::check`).
- Messages from the install code ("NERDS Photo is running. Quit it…") name the app as installed.
- The pictures on each app's page are the upstream screenshots.

## Next

1. Identity: the `ai.storyteller.*` bundle ids and the `artcraft-launcher` machine names
   (binary, crates, files) → NERDS ones.
2. Code signing (Apple Developer ID, a Windows certificate), or recipients keep clicking through a
   one-time "unidentified developer" warning, and the Windows launcher can't update itself.
3. NERDS's own website and Discord: fill in the links in `catalog.rs`, and point the apps' Help
   menus at them (a patch in each app repository's `nerds.json`).
