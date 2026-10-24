# NERDS Launcher app icon

The app icon is the **white NERDS N mark on a NERDS-blue (#0077C0) tile**:
[`docs/brand/nerds-app-icon.svg`](../../docs/brand/nerds-app-icon.svg). Every file in this
directory is rendered from it by `packaging/icons.sh`. It is a NERDS brand asset, not covered by
the code's MIT or Apache-2.0 license: [`docs/brand/LICENSE-brand.txt`](../../docs/brand/LICENSE-brand.txt).

## Files

- `artcraft-launcher.svg`: a copy of `docs/brand/nerds-app-icon.svg`; also the hicolor scalable icon.
- `artcraft-launcher-1024.png`: 1024 px render, with extra transparent margin for Apple's icon grid.
- `artcraft-launcher.icns`: macOS bundle icon (`CFBundleIconFile`).
- `artcraft-launcher.ico`: Windows icon, 16–256 px, embedded in the `.exe` by
  `apps/artcraft-launcher/build.rs` and used by the MSI.
- `hicolor/<size>/apps/ai.storyteller.artcraft-launcher.png` (16–512) and
  `hicolor/scalable/apps/ai.storyteller.artcraft-launcher.svg`: Linux icon theme.
- The tray / menu-bar icon: `docs/brand/nerds-mark-black-44.png` (the black N mark, the macOS
  template, 22 pt @2x) and `docs/brand/nerds-tray-64.png` (the app icon; Windows, Linux, FreeBSD).

The file names still carry the upstream `artcraft-launcher` machine name; NERDS.md lists that
rename as a later step.

## Regenerate

Run `packaging/icons.sh` on any OS. It needs `resvg`; the `.ico` and `.icns` are packed by
`cargo xtask ico` / `icns`. To change the artwork, edit `docs/brand/source/make_icons.py` (or the
SVGs it writes) and run the script again.
