#!/usr/bin/env bash
# Regenerate every app and tray icon from the NERDS files in docs/brand/.
#
# - App icon (Dock, taskbar, Start menu, Linux menus, installers): docs/brand/nerds-app-icon.svg,
#   the white N mark on a NERDS-blue tile.
# - Tray: the black N mark as the macOS menu-bar template; the app icon on Windows, Linux and BSD.
#
# Needs: resvg (brew install resvg / cargo install resvg). The .ico and .icns are packed by
# `cargo xtask ico` / `cargo xtask icns`, so this runs on any OS. The outputs are committed, so
# packaging (and the Rust build, which include_bytes!s some of them) never needs these tools.
# The SVGs themselves are drawn by docs/brand/source/make_logo.py and make_icons.py.
#
#   packaging/icons.sh
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIR="$ROOT/assets/app-icon"
BRAND="$ROOT/docs/brand"
SVG="$BRAND/nerds-app-icon.svg"
ID="ai.storyteller.artcraft-launcher"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

command -v resvg >/dev/null || { echo "error: resvg not found (brew install resvg)" >&2; exit 1; }

# macOS icons sit on Apple's 824/1024 icon grid: widen the view box round the full-bleed tile.
BOX='viewBox="0 0 1024 1024"'
grep -q "$BOX" "$SVG" || { echo "error: expected $BOX in $SVG" >&2; exit 1; }
MAC="$TMP/mac.svg"
sed "s/$BOX/viewBox=\"-124.25 -124.25 1272.5 1272.5\"/" "$SVG" >"$MAC"
FLAT="$SVG"

render() {
  local src="$1" size="$2" out="$3"
  resvg -w "$size" -h "$size" "$src" "$out" </dev/null
}

render "$MAC" 1024 "$DIR/artcraft-launcher-1024.png"

# Linux hicolor theme.
for s in 16 24 32 48 64 128 256 512; do
  mkdir -p "$DIR/hicolor/${s}x${s}/apps"
  render "$FLAT" "$s" "$DIR/hicolor/${s}x${s}/apps/$ID.png"
done
mkdir -p "$DIR/hicolor/scalable/apps"
cp "$SVG" "$DIR/hicolor/scalable/apps/$ID.svg"
cp "$SVG" "$DIR/artcraft-launcher.svg"

# Windows .ico.
ICO_PNGS=()
for s in 16 20 24 32 40 48 64 128 256; do
  render "$FLAT" "$s" "$TMP/ico-$s.png"
  ICO_PNGS+=("$TMP/ico-$s.png")
done
(cd "$ROOT" && cargo run -q -p xtask -- ico "$DIR/artcraft-launcher.ico" "${ICO_PNGS[@]}")

# macOS .icns.
ICNS_PNGS=()
for s in 16 32 64 128 256 512 1024; do
  render "$MAC" "$s" "$TMP/icns-$s.png"
  ICNS_PNGS+=("$TMP/icns-$s.png")
done
(cd "$ROOT" && cargo run -q -p xtask -- icns "$DIR/artcraft-launcher.icns" "${ICNS_PNGS[@]}")

# Tray icons: the black N mark is the macOS menu-bar template (44 px = 22 pt @2x; macOS tints it);
# the app icon is for the Windows notification area and Linux/BSD status notifiers.
render "$BRAND/nerds-mark-black-square.svg" 44 "$BRAND/nerds-mark-black-44.png"
render "$SVG" 64 "$BRAND/nerds-tray-64.png"

# The apps' icons in the launcher (128 px), from their NERDS sources.
for src in "$ROOT"/assets/app-icons/source/*.svg; do
  render "$src" 128 "$ROOT/assets/app-icons/$(basename "$src" .svg).png"
done

# The white N.E.R.D.S. letters on the Windows installer's art (packaging/windows/package.ps1).
resvg -w 960 "$BRAND/nerds-wordmark-white.svg" "$BRAND/nerds-wordmark-white.png" </dev/null
echo "icons written to $DIR"
