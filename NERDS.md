# NERDS edition: what changed and what's next

This repository is ArtCraft Launcher reskinned for NERDS. It is a modified version of
[storytold/craft-launcher](https://github.com/storytold/craft-launcher) (MIT OR Apache-2.0); this
file lists the changes, as the Apache License asks of modified files.

## Stage 1: the launcher (this change)

**Look**
- Colours: `crates/ui-egui/src/theme.rs`. NERDS blue `#0077C0` (sampled from the NERDS logo) for
  accents and primary buttons; deep navy `#0D1824` surfaces in dark mode, cool white `#F5F7FA` in
  light mode. Status colours (green, amber, red) are unchanged. The web build's loading page
  (`apps/artcraft-launcher-web/index.html`) matches.
- Header logo: the NERDS N mark and N.E.R.D.S. side by side (`docs/brand/nerds-logo-horizontal*.svg`).
- App icon, tray icon, Windows installer art and the macOS DMG background: NERDS.
- App icons: one NERDS-blue tile per app with a white symbol for what it does
  (`assets/app-icons/source/*.svg`, rendered by `packaging/icons.sh`).

**Names**
- NERDS Launcher, the NERDS Suite, and each app as "NERDS" + what it does
  (`crates/core/src/catalog.rs`, field `display`). The app's original name stays in `name`,
  because the install and detection code must match what the app's installer calls itself.
- All user-facing text, in all eleven languages, and the packaging (MSI, DMG, .deb/.rpm, Flatpak)
  say NERDS Launcher. The Windows installer has its own upgrade code and component IDs, so it
  never replaces or clashes with an ArtCraft Launcher install.

**Switched off until NERDS has its own**
- Website, GitHub, Discord and social links: empty in `catalog.rs`; an empty link is hidden in
  the footer, Settings and the tray menu. Fill them in to show them.
- Self-update (`LAUNCHER_SELF_UPDATE`): off, or it would offer the upstream ArtCraft Launcher as
  an update. Turn it on once `LAUNCHER_REPO` points at the NERDS Launcher's own repository.
- The upstream release feed's announcements and alerts (`UPSTREAM_NOTICES`): off, as they are
  ArtCraft's news. Release information from the feed is still used.

**Still upstream in stage 1**
- The launcher installs the official ArtCraft builds from `github.com/storytold/<app>`. They open
  under their own names (PhotoCraft, …) and icons; the launcher shows them as NERDS apps.
- Messages from the install code ("PhotoCraft is running. Quit it…") name the app as installed.
- The pictures on each app's page are the upstream screenshots.

## Stage 2: NERDS-branded apps

1. Fork the 13 app repositories and this one to the NERDS GitHub account or organization.
2. In each app: product name "NERDS <X>" (window title, menus, About, installer metadata, bundle
   id, Windows product name), the NERDS icon from `assets/app-icons/source/`, and a NERDS colour
   theme as the default. Releases built by each fork's own GitHub Actions workflow.
3. Here: `GITHUB_OWNER` and every `repo` → the NERDS account; `name` → the NERDS name, with the
   upstream name kept in `former_names` so existing installs are still found; the release feed
   off (it only knows the upstream releases); `LAUNCHER_REPO` → the NERDS fork and
   `LAUNCHER_SELF_UPDATE` on.
4. Identity: the publisher (`Learning Machines LLC` in `build.rs`, the `.wxs`, `nfpm.yaml`,
   the metainfo and `self_update.rs`), the `ai.storyteller.*` bundle ids, and the
   `artcraft-launcher` machine names (binary, crates, files) → NERDS ones.
5. macOS: regenerate `packaging/macos/dmg/background.tiff` and `dmg-layout.DS_Store` with
   `packaging/macos/dmg/generate.py` (the volume is now "NERDS Launcher").
6. Code signing (Apple Developer ID, a Windows certificate), or recipients click through a
   one-time "unidentified developer" warning.
