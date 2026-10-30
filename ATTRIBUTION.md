# Attribution

Every non-code asset in this repository, with its author, source and license. NERDS
Launcher's code (ArtCraft Launcher's, with the NERDS changes) is MIT OR Apache-2.0 (see [`LICENSE-MIT`](LICENSE-MIT),
[`LICENSE-APACHE`](LICENSE-APACHE) and [`NOTICE`](NOTICE)). When you add an asset, add a row here
in the same change; third-party assets must be permissively licensed and keep their license file
next to them.

## Bundled in the app

| Path | Title | Author | Source | License |
|---|---|---|---|---|
| `assets/app-icon/` (all files) | NERDS Launcher app icon: the white N mark on a NERDS-blue tile | NERDS | Rendered from `docs/brand/nerds-app-icon.svg` by `packaging/icons.sh` | NERDS brand asset, not MIT/Apache: [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt) |
| `packaging/macos/dmg/` (all files) | macOS DMG window background and layout | ArtCraft Launcher contributors; recoloured for NERDS, with the NERDS N mark | Original work, see [`packaging/macos/dmg/README.md`](packaging/macos/dmg/README.md) | MIT OR Apache-2.0, except the N mark ([`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt)) |
| `assets/social/linkedin.svg` | LinkedIn logo, linking to ArtCraft's company page | Fonticons, Inc. ([Font Awesome Free](https://fontawesome.com) 6.7.2) | Font Awesome Free `svgs/brands/linkedin.svg`; white fill and a square view box, see [`assets/social/README.md`](assets/social/README.md) | CC BY 4.0. The logo is a trademark of LinkedIn Corporation. |
| `assets/social/x.svg`, `youtube.svg`, `tiktok.svg`, `instagram.svg`, `discord.svg` | X, YouTube, TikTok, Instagram and Discord logos, linking to ArtCraft's accounts | [Simple Icons](https://simpleicons.org/) contributors | Simple Icons, `fill` set to white for tinting; see [`assets/social/README.md`](assets/social/README.md) | CC0 1.0. The logos are trademarks of X Corp., Google LLC, TikTok / ByteDance, Meta Platforms, Inc. and Discord Inc. |
| `assets/social/github.svg`, `assets/social/globe.svg` | GitHub logo (linking to ArtCraft's GitHub) and a globe (linking to ArtCraft's website) | [Simple Icons](https://simpleicons.org/) contributors; [Lucide](https://lucide.dev) contributors | Simple Icons `github.svg` and Lucide `globe.svg` (lucide-static 0.468.0), white for tinting; see [`assets/social/README.md`](assets/social/README.md) | CC0 1.0 (GitHub logo; a trademark of GitHub, Inc.); ISC (globe) |
| (not bundled) the pictures on each app's details page | Screenshots of the Craft Apps, fetched when a page is opened and kept in the launcher's cache folder | The apps' contributors | [storytold/craft-launcher-assets](https://github.com/storytold/craft-launcher-assets), pinned to a tag (`SCREENSHOTS_REV` in `crates/core/src/details.rs`); each picture's source and license are listed in that repository's `ATTRIBUTION.md` | Each app's own license for the interface (MIT OR Apache-2.0); the work shown in them is public domain or made in the app, as that file says |
| `docs/brand/nerds-logo-horizontal.svg`, `nerds-logo-horizontal-dark.svg` | NERDS logo side by side (N mark and N.E.R.D.S.), shown in the window header | NERDS | Redrawn as vectors from the PNG logo NERDS supplied (`docs/brand/source/make_logo.py`) | NERDS brand asset, not MIT/Apache: [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt) |
| `docs/brand/nerds-tray-64.png`, `nerds-mark-black-44.png`, `nerds-app-icon.svg` | The tray / menu-bar icon and the in-app icon of NERDS Launcher | NERDS | Rendered from `docs/brand/nerds-app-icon.svg` and `nerds-mark-black-square.svg` by `packaging/icons.sh` | NERDS brand asset, not MIT/Apache: [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt) |
| `assets/fonts/Inter-Variable.ttf` | Inter 4.001, variable (body text) | The Inter Project Authors (Rasmus Andersson) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/inter> (`Inter[opsz,wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-Inter.txt`](assets/fonts/OFL-Inter.txt) |
| `assets/fonts/Archivo-Variable.ttf` | Archivo 2.001, variable (headings and app names) | The Archivo Project Authors (Omnibus-Type) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/archivo> (`Archivo[wdth,wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-Archivo.txt`](assets/fonts/OFL-Archivo.txt) |
| `assets/fonts/GeistMono-Variable.ttf` | Geist Mono 1.701, variable (labels and buttons) | The Geist Project Authors (Vercel) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/geistmono> (`GeistMono[wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-GeistMono.txt`](assets/fonts/OFL-GeistMono.txt) |
| (dependency) `epaint_default_fonts/fonts/NotoEmoji-Regular.ttf` | Noto Emoji Regular, monochrome emoji fallback for arbitrary release-note, feed and alert text | The Noto Project Authors (Google) | Bundled transitively by `egui 0.37`'s `monochrome_emoji_fonts` feature from `epaint_default_fonts`; <https://github.com/storytold/egui/tree/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts> | SIL OFL 1.1, `epaint_default_fonts/fonts/OFL.txt`; see also the dependency `README` and egui `README.md` |
| (dependency) `epaint_default_fonts/fonts/emoji-icon-font.ttf` | emoji-icon-font, monochrome symbol/emoji fallback | John Slegers | Bundled transitively by `egui 0.37`'s `monochrome_emoji_fonts` feature from `epaint_default_fonts`; <https://github.com/jslegers/emoji-icon-font> | MIT, Copyright (c) 2014 John Slegers, `epaint_default_fonts/fonts/emoji-icon-font-mit-license.txt` |
| (dependency) `epaint_default_fonts/fonts/egui-icons.ttf` | egui-icons, five-glyph subset of emoji-icon-font | John Slegers | [egui 6d518fc7](https://github.com/storytold/egui/tree/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts); included by default_fonts independently of monochrome_emoji_fonts | MIT, Copyright (c) 2014 John Slegers; full upstream text in `NOTICE` |
| (dependency) `epaint_default_fonts/fonts/Ubuntu-Light.ttf` | Ubuntu Light, default font fallback | Canonical Ltd. | [Pinned egui source](https://github.com/storytold/egui/tree/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts) | Ubuntu Font Licence 1.0; full upstream text and copyright in `NOTICE` |
| (dependency) `epaint_default_fonts/fonts/Hack-Regular.ttf` | Hack Regular, default monospace fallback | Source Foundry Authors; Bitstream Inc.; DejaVu contributors | [Pinned egui source](https://github.com/storytold/egui/tree/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts) | MIT and Bitstream Vera License; DejaVu work public domain; full upstream notices in `NOTICE` |

### App icons

The launcher shows each app with a NERDS icon: a tile in the Craft app's colour with a white
symbol for what the app does. They were drawn for NERDS (`docs/brand/source/make_icons.py`); the SVG sources
are in `assets/app-icons/source/` and `packaging/icons.sh` renders the 128 px PNGs from them.

| Path | Title | Author | Source | License |
|---|---|---|---|---|
| `assets/app-icons/*.png`, `assets/app-icons/source/*.svg` | The NERDS app icons (Photo, Vector, Film, Light, PDF, Effects, Layout, Proto, Word, Sound, Deck, CAD, Grid), 128 px | NERDS | Drawn for NERDS | NERDS brand assets, not MIT/Apache: [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt) |

## Documentation and README

| Path | Title | Author | Source | License |
|---|---|---|---|---|
| `docs/brand/` (all files) | NERDS name, N mark and N.E.R.D.S. lettering, and the scripts that draw them | NERDS | Supplied by NERDS as PNG; redrawn as vectors here | NERDS brand assets, not MIT/Apache: [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt) |
| README app icons (linked, not copied) | Each Craft app's 64 px icon | ArtCraft Team and each app's contributors | `raw.githubusercontent.com/storytold/<app>/main/assets/app-icon/hicolor/64x64/apps/ai.storyteller.<app>.png` | MIT OR Apache-2.0 (each app's license) |
