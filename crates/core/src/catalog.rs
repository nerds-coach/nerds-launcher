//! The NerdsCraft Suite: the thirteen apps the launcher knows about, under their NERDS names and
//! colours.
//!
//! The list is data so a later version can fetch it from an API; [`SUITE`] is the built-in copy.
//!
//! Text here is English, the source the UI translates from: each `tagline` has a catalog row
//! keyed by the app id (`tr_ctx(app.id, app.tagline)` in `ui-egui`).

/// Who makes an app. Every NERDS app is `Official` (a NERDS build of an official Craft App); a
/// community app is maintained elsewhere and is labelled as such wherever it is shown (upstream
/// lists one, SolidCraft, which the NERDS suite leaves out).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppKind {
    Official,
    Community,
}

/// One app in the suite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CraftApp {
    /// Machine name: repo, binary, package and asset stem (`photocraft`).
    pub id: &'static str,
    /// Its NERDS name (`NERDS Photo`): what the launcher shows, and the name its macOS bundle
    /// (`NERDS Photo.app`) and Windows Installer product carry. Install and detection code matches
    /// on this and on [`CraftApp::former_names`].
    pub name: &'static str,
    /// What it's for, in a few words (English; the UI translates it by app id).
    pub tagline: &'static str,
    /// Brand colour (sRGB), for surfaces: its Craft app's colour, as its NERDS icon has.
    pub color: [u8; 3],
    /// The brand colour for text on a light background (the site's light `--app-ink`).
    pub ink_light: [u8; 3],
    /// The brand colour for text on a dark background (the site's dark `--app-ink`).
    pub ink_dark: [u8; 3],
    /// GitHub `owner/repo`: the NERDS build of the app (`nerds-coach/nerds-photo`).
    pub repo: &'static str,
    /// Older names its release assets may carry (PdfCraft shipped as PrintCraft until 0.2.1).
    pub former_ids: &'static [&'static str],
    /// Older names it installed under: the upstream name the NERDS builds used until they took
    /// their NERDS names (`PhotoCraft`), which is also the name its data folders, Windows registry
    /// key and Mac `<Name>Version` key keep, and PdfCraft's PrintCraft.
    pub former_names: &'static [&'static str],
    pub kind: AppKind,
    /// Who publishes it, as the card's byline shows it (`NERDS`).
    pub publisher: &'static str,
    /// The publisher its Windows installer names, besides the NERDS builds' `NERDS Craft` and the
    /// earlier `Learning Machines LLC` (`detect.rs`), for an app that has another.
    pub msi_publisher: Option<&'static str>,
    /// The bundle id when it isn't `ai.storyteller.<id>` (a community app brings its own).
    pub bundle: Option<&'static str>,
    /// Older bundle ids its Flatpak shipped under (SolidCraft was `ai.storyteller.solidcraft`).
    pub former_bundles: &'static [&'static str],
    /// Its website, when it isn't a page on getartcraft.com (a community app has its own).
    pub site: Option<&'static str>,
    /// Whether the launcher installs its macOS build. `false` while those builds carry no
    /// Developer ID signature the launcher can check: it then opens a copy that's there and
    /// links to the releases, but installs nothing.
    pub installs_on_mac: bool,
}

impl CraftApp {
    /// `ai.storyteller.<id>`: macOS bundle id, Flatpak id and Linux desktop-file id.
    pub fn bundle_id(&self) -> String {
        self.bundle.map_or_else(|| format!("ai.storyteller.{}", self.id), str::to_string)
    }

    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// The app's page: its own site when it has one, else its NERDS repository (NERDS has no page
    /// per app yet).
    pub fn website(&self) -> String {
        self.site.map_or_else(|| self.repo_url(), str::to_string)
    }

    /// Whether the app has a page of its own, apart from its repository (the UI leaves out a
    /// Website link that would only repeat the GitHub one).
    pub fn has_own_website(&self) -> bool {
        self.website() != self.repo_url()
    }

    /// Whether the launcher installs this app from a `kind` package.
    pub fn installs(&self, kind: crate::PackageKind) -> bool {
        kind != crate::PackageKind::Dmg || self.installs_on_mac
    }

    /// Bundle ids this app may be installed under, current first.
    pub fn bundle_ids(&self) -> impl Iterator<Item = String> + '_ {
        std::iter::once(self.bundle_id()).chain(self.former_bundles.iter().map(|b| b.to_string()))
    }

    /// Asset stems this app's releases may use, current first.
    pub fn stems(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.id).chain(self.former_ids.iter().copied())
    }

    /// Names this app's installers may have registered, current (NERDS) first.
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.name).chain(self.former_names.iter().copied())
    }

    pub fn by_id(id: &str) -> Option<&'static CraftApp> {
        SUITE.iter().find(|a| a.id == id)
    }
}

const fn app(
    id: &'static str,
    name: &'static str,
    former_names: &'static [&'static str],
    tagline: &'static str,
    [color, ink_light, ink_dark]: [[u8; 3]; 3],
    repo: &'static str,
) -> CraftApp {
    CraftApp {
        id,
        name,
        tagline,
        color,
        ink_light,
        ink_dark,
        repo,
        former_ids: &[],
        former_names,
        kind: AppKind::Official,
        publisher: "NERDS",
        msi_publisher: None,
        bundle: None,
        former_bundles: &[],
        site: None,
        installs_on_mac: true,
    }
}

/// NERDS blue (#0077C0), sampled from the NERDS logo: the launcher's own colour.
pub const NERDS_BLUE: [u8; 3] = [0x00, 0x77, 0xc0];
/// NERDS blue darkened for text on a light background (6:1 on the light page).
pub const NERDS_INK_LIGHT: [u8; 3] = [0x00, 0x5e, 0x99];
/// NERDS blue lightened for text on a dark background (8:1 on the dark page).
pub const NERDS_INK_DARK: [u8; 3] = [0x5c, 0xb6, 0xf2];

/// The suite, in display order. `name` is the NERDS name each build installs as and the launcher
/// shows; `former_names` starts with the upstream app's name, which earlier NERDS builds installed
/// as. Each app keeps its Craft app's colour (`[surface, ink on light, ink on dark]`), as its NERDS
/// icon does. `repo` is the NERDS build of the app: the upstream release with the NERDS icon and
/// names (`nerds-coach/nerds-*`).
pub const SUITE: [CraftApp; 13] = [
    app(
        "photocraft",
        "NERDS Photo",
        &["PhotoCraft"],
        "Image editing",
        [[0x2f, 0x7b, 0xf5], [0x1a, 0x5b, 0xc4], [0x7f, 0xb0, 0xff]],
        "nerds-coach/nerds-photo",
    ),
    app(
        "vectorcraft",
        "NERDS Vector",
        &["VectorCraft"],
        "Vector illustration",
        [[0xe8, 0x57, 0x3f], [0xb8, 0x3a, 0x24], [0xff, 0x8a, 0x70]],
        "nerds-coach/nerds-vector",
    ),
    app(
        "filmcraft",
        "NERDS Film",
        &["FilmCraft"],
        "Video editing, color and sound",
        [[0x8b, 0x5c, 0xf6], [0x6a, 0x3f, 0xd6], [0xb6, 0x9c, 0xff]],
        "nerds-coach/nerds-film",
    ),
    app(
        "lightcraft",
        "NERDS Light",
        &["LightCraft"],
        "Photo library and raw development",
        [[0xf2, 0xa5, 0x16], [0x8a, 0x58, 0x00], [0xff, 0xc1, 0x4d]],
        "nerds-coach/nerds-light",
    ),
    CraftApp {
        former_ids: &["printcraft"],
        ..app(
            "pdfcraft",
            "NERDS PDF",
            &["PdfCraft", "PrintCraft"],
            "Reading, organizing and protecting PDFs",
            [[0x12, 0xa5, 0x8a], [0x0a, 0x75, 0x63], [0x3f, 0xd6, 0xb8]],
            "nerds-coach/nerds-pdf",
        )
    },
    app(
        "effectcraft",
        "NERDS Effects",
        &["EffectCraft"],
        "Motion graphics and visual effects",
        [[0xe0, 0x36, 0x8f], [0xb0, 0x20, 0x6c], [0xff, 0x7a, 0xb8]],
        "nerds-coach/nerds-effects",
    ),
    app(
        "designcraft",
        "NERDS Layout",
        &["DesignCraft"],
        "Page layout and publishing",
        [[0x7b, 0xb5, 0x1c], [0x45, 0x70, 0x08], [0xa6, 0xe0, 0x4a]],
        "nerds-coach/nerds-layout",
    ),
    app(
        "figcraft",
        "NERDS Proto",
        &["FigCraft"],
        "Interface design and prototyping",
        [[0x02, 0xda, 0x96], [0x04, 0x7a, 0x56], [0x4f, 0xe8, 0xb6]],
        "nerds-coach/nerds-proto",
    ),
    app(
        "wordcraft",
        "NERDS Word",
        &["WordCraft"],
        "Writing and document design",
        [[0x3f, 0x5f, 0xe0], [0x2a, 0x44, 0xb8], [0x8e, 0xa2, 0xff]],
        "nerds-coach/nerds-word",
    ),
    app(
        "soundcraft",
        "NERDS Sound",
        &["SoundCraft"],
        "Recording, editing and mixing audio",
        [[0x1f, 0xa5, 0xc9], [0x0e, 0x72, 0x90], [0x5f, 0xd0, 0xee]],
        "nerds-coach/nerds-sound",
    ),
    app(
        "deckcraft",
        "NERDS Deck",
        &["DeckCraft"],
        "Presentations and slide shows",
        [[0xf0, 0x7a, 0x1e], [0xa8, 0x4f, 0x0a], [0xff, 0xa0, 0x60]],
        "nerds-coach/nerds-deck",
    ),
    app(
        "cadcraft",
        "NERDS CAD",
        &["CADCraft"],
        "Computer-aided design and drafting",
        [[0x3e, 0xa6, 0xe8], [0x1a, 0x6f, 0xa8], [0x7c, 0xc8, 0xff]],
        "nerds-coach/nerds-cad",
    ),
    app(
        "gridcraft",
        "NERDS Grid",
        &["GridCraft"],
        "Spreadsheets and data",
        [[0x1f, 0x9d, 0x55], [0x12, 0x70, 0x3a], [0x4f, 0xd6, 0x8a]],
        "nerds-coach/nerds-grid",
    ),
];

/// Upstream's community app, which the NERDS suite leaves out (it has no NERDS build). It stays
/// described here so the code for community apps (their own publisher, bundle ids, a macOS
/// build the launcher can't verify) keeps its tests. Not part of [`SUITE`].
pub const SOLIDCRAFT: CraftApp = CraftApp {
    kind: AppKind::Community,
    publisher: "Tangency Labs",
    msi_publisher: Some("SolidCraft contributors"),
    bundle: Some("io.tangencylabs.solidcraft"),
    former_bundles: &["ai.storyteller.solidcraft", "ai.storyteller.solvecraft"],
    site: Some("https://solidcraft.io"),
    installs_on_mac: false,
    ..app(
        "solidcraft",
        "SolidCraft",
        &[],
        "Parametric 3D CAD",
        [[0x46, 0xa1, 0xfb], [0x16, 0x53, 0xae], [0x8c, 0xc4, 0xff]],
        "tangency-labs/solidcraft",
    )
};

/// The GitHub account the NERDS builds live under: every app's `repo` and the launcher's own.
pub const NERDS_OWNER: &str = "nerds-coach";

/// Whether to ask ArtCraft's release feed (`feed.rs`). Off: it describes the upstream releases in
/// ArtCraft's repositories, not the NERDS builds, so every app is asked about on GitHub.
pub const USE_RELEASE_FEED: bool = false;

/// The Apple Developer team whose signature a Mac app (or the launcher's own update) must carry
/// before it is installed. `None` while the NERDS builds are ad-hoc signed: the bundle's
/// signature must then be intact, whoever made it (`install::macos::check`).
pub const APP_SIGNING_TEAM: Option<&str> = None;

// NERDS links. An empty URL hides that link everywhere (footer, Settings, tray menu); fill these
// in once NERDS has them.

/// The NERDS website.
pub const WEBSITE_URL: &str = "";
/// The GitHub account of the upstream apps, which the release feed describes: the feed can move
/// an app's repository within it, never out of it (`feed.rs`; the feed is off, `USE_RELEASE_FEED`).
pub const GITHUB_OWNER: &str = "storytold";
/// NERDS on GitHub.
pub const GITHUB_URL: &str = "https://github.com/nerds-coach";
/// NERDS's social accounts: (service, URL), shown as small icons. The names pick the icon in
/// `ui-egui`'s `icons::social` (X, Instagram, LinkedIn, YouTube, TikTok).
pub const SOCIAL: [(&str, &str); 0] = [];

/// Whether a NERDS link is set: an empty URL hides that link.
pub fn has_link(url: &str) -> bool {
    !url.is_empty()
}

/// The NERDS community on Discord (empty: no Discord link).
pub const DISCORD_URL: &str = "";
/// Deep link into the Discord application for the same invite (empty with [`DISCORD_URL`]).
pub const DISCORD_APP_URL: &str = "";

/// NERDS Launcher itself, described like a Craft App so it can update itself with the same
/// machinery (asset names `artcraft-launcher-<ver>-…`, bundle id `ai.storyteller.artcraft-launcher`,
/// `NERDS Launcher.app`). Not part of [`SUITE`].
pub const LAUNCHER: CraftApp = app(
    "artcraft-launcher",
    "NERDS Launcher",
    &[],
    "Installs and updates the NerdsCraft Suite",
    [NERDS_BLUE, NERDS_INK_LIGHT, NERDS_INK_DARK],
    "nerds-coach/nerds-launcher",
);

/// The launcher's own repo, checked so it can say when a newer launcher is out.
pub const LAUNCHER_REPO: &str = "nerds-coach/nerds-launcher";
/// Whether to show the upstream release feed's announcements and alerts. Off: they are written by
/// and about ArtCraft (its news, its links), not NERDS.
pub const UPSTREAM_NOTICES: bool = false;
/// Whether the launcher looks for newer versions of itself, in [`LAUNCHER_REPO`] (the NERDS
/// Launcher's own releases, never the upstream ArtCraft Launcher's).
pub const LAUNCHER_SELF_UPDATE: bool = true;
/// The launcher's own asset stem.
pub const LAUNCHER_ID: &str = "artcraft-launcher";
/// The stem of the NERDS Launcher's release files (`nerds-launcher-<version>-windows-x64.msi`…;
/// `packaging/windows/package.ps1`, `packaging/macos/package.sh`). Self-update looks for these
/// first, then for the upstream `artcraft-launcher-…` names.
pub const LAUNCHER_ASSET_STEM: &str = "nerds-launcher";

/// An app's 128 px NERDS icon (PNG) in `assets/app-icons` (sources in `assets/app-icons/source`): the UI
/// draws it, and the Linux install puts it in the desktop menu.
pub fn app_icon_png(id: &str) -> Option<&'static [u8]> {
    Some(match id {
        "photocraft" => include_bytes!("../../../assets/app-icons/photocraft.png"),
        "vectorcraft" => include_bytes!("../../../assets/app-icons/vectorcraft.png"),
        "filmcraft" => include_bytes!("../../../assets/app-icons/filmcraft.png"),
        "lightcraft" => include_bytes!("../../../assets/app-icons/lightcraft.png"),
        "pdfcraft" => include_bytes!("../../../assets/app-icons/pdfcraft.png"),
        "effectcraft" => include_bytes!("../../../assets/app-icons/effectcraft.png"),
        "designcraft" => include_bytes!("../../../assets/app-icons/designcraft.png"),
        "wordcraft" => include_bytes!("../../../assets/app-icons/wordcraft.png"),
        "deckcraft" => include_bytes!("../../../assets/app-icons/deckcraft.png"),
        "gridcraft" => include_bytes!("../../../assets/app-icons/gridcraft.png"),
        "cadcraft" => include_bytes!("../../../assets/app-icons/cadcraft.png"),
        "soundcraft" => include_bytes!("../../../assets/app-icons/soundcraft.png"),
        "figcraft" => include_bytes!("../../../assets/app-icons/figcraft.png"),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_app_has_its_icon() {
        for app in &SUITE {
            let png = app_icon_png(app.id).unwrap_or_else(|| panic!("no icon for {}", app.id));
            assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"), "{} icon isn't a PNG", app.id);
        }
        assert!(app_icon_png("notacraft").is_none());
    }

    #[test]
    fn suite_has_thirteen_apps_with_unique_ids() {
        assert_eq!(SUITE.len(), 13);
        let mut ids: Vec<_> = SUITE.iter().map(|a| a.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), SUITE.len());
        for a in &SUITE {
            assert!(a.name.starts_with("NERDS "), "{} has no NERDS name", a.id);
            assert_eq!(a.kind, AppKind::Official, "{}", a.id);
            assert!(a.repo.starts_with(&format!("{NERDS_OWNER}/nerds-")), "{} isn't a NERDS build: {}", a.id, a.repo);
            // The upstream name comes first among the former names: the NERDS builds installed
            // under it until they took their NERDS names, and its data folders still carry it.
            assert_eq!(a.former_names.first().map(|n| n.to_lowercase()).as_deref(), Some(a.id), "{}", a.id);
        }
        let mut repos: Vec<_> = SUITE.iter().map(|a| a.repo).collect();
        repos.sort_unstable();
        repos.dedup();
        assert_eq!(repos.len(), SUITE.len(), "two apps share a repository");
        assert!(LAUNCHER.repo.starts_with(NERDS_OWNER));
        let mut shown: Vec<_> = SUITE.iter().map(|a| a.name).collect();
        shown.sort_unstable();
        shown.dedup();
        assert_eq!(shown.len(), SUITE.len(), "two apps share a NERDS name");
    }

    #[test]
    fn apps_answer_to_their_nerds_and_upstream_names() {
        let photo = CraftApp::by_id("photocraft").unwrap();
        assert_eq!(photo.names().collect::<Vec<_>>(), ["NERDS Photo", "PhotoCraft"]);
        assert_eq!(LAUNCHER.names().collect::<Vec<_>>(), ["NERDS Launcher"]);
    }

    #[test]
    fn pdfcraft_also_answers_to_printcraft() {
        let pdf = CraftApp::by_id("pdfcraft").unwrap();
        assert_eq!(pdf.stems().collect::<Vec<_>>(), ["pdfcraft", "printcraft"]);
        assert_eq!(pdf.names().collect::<Vec<_>>(), ["NERDS PDF", "PdfCraft", "PrintCraft"]);
        assert_eq!(pdf.website(), "https://github.com/nerds-coach/nerds-pdf");
        assert_eq!(CraftApp::by_id("wordcraft").unwrap().website(), "https://github.com/nerds-coach/nerds-word");
    }
}
