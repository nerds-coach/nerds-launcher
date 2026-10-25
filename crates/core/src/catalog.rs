//! The NERDS Suite: the thirteen apps the launcher knows about (the NERDS Suite's apps, shown
//! under NERDS names and colours).
//!
//! The list is data so a later version can fetch it from an API; [`SUITE`] is the built-in copy.
//!
//! Text here is English, the source the UI translates from: each `tagline` has a catalog row
//! keyed by the app id (`tr_ctx(app.id, app.tagline)` in `ui-egui`).

/// One app in the suite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CraftApp {
    /// Machine name: repo, binary, package and asset stem (`photocraft`).
    pub id: &'static str,
    /// The name the app installs under: its macOS bundle, Windows Installer product and registry
    /// key (`PhotoCraft`). Install and detection code matches on this, so it must be the name the
    /// release's own installer uses.
    pub name: &'static str,
    /// The NERDS name the launcher shows (`NERDS Photo`). While the launcher installs the
    /// upstream builds this differs from [`CraftApp::name`]; once the NERDS builds ship, both match.
    pub display: &'static str,
    /// What it's for, in a few words (English; the UI translates it by app id).
    pub tagline: &'static str,
    /// Brand colour (sRGB), for surfaces: NERDS blue for every app.
    pub color: [u8; 3],
    /// The brand colour for text on a light background (the site's light `--app-ink`).
    pub ink_light: [u8; 3],
    /// The brand colour for text on a dark background (the site's dark `--app-ink`).
    pub ink_dark: [u8; 3],
    /// GitHub `owner/repo`.
    pub repo: &'static str,
    /// Older names its release assets may carry (PdfCraft shipped as PrintCraft until 0.2.1).
    pub former_ids: &'static [&'static str],
    /// Older display names its installers registered (PdfCraft's MSI said PrintCraft until 0.2.1).
    pub former_names: &'static [&'static str],
}

impl CraftApp {
    /// `ai.storyteller.<id>`: macOS bundle id, Flatpak id and Linux desktop-file id.
    pub fn bundle_id(&self) -> String {
        format!("ai.storyteller.{}", self.id)
    }

    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// The app's page: its source repository until NERDS has a page per app.
    pub fn website(&self) -> String {
        self.repo_url()
    }

    /// Whether the app has a page of its own, apart from its repository (the UI leaves out a
    /// Website link that would only repeat the GitHub one).
    pub fn has_own_website(&self) -> bool {
        self.website() != self.repo_url()
    }

    /// Asset stems this app's releases may use, current first.
    pub fn stems(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.id).chain(self.former_ids.iter().copied())
    }

    /// Display names this app's installers may have registered, current first.
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.name).chain(self.former_names.iter().copied())
    }

    pub fn by_id(id: &str) -> Option<&'static CraftApp> {
        SUITE.iter().find(|a| a.id == id)
    }
}

const fn app(id: &'static str, name: &'static str, display: &'static str, tagline: &'static str, repo: &'static str) -> CraftApp {
    CraftApp {
        id,
        name,
        display,
        tagline,
        color: NERDS_BLUE,
        ink_light: NERDS_INK_LIGHT,
        ink_dark: NERDS_INK_DARK,
        repo,
        former_ids: &[],
        former_names: &[],
    }
}

/// NERDS blue (#0077C0), sampled from the NERDS logo: every app's surface colour.
pub const NERDS_BLUE: [u8; 3] = [0x00, 0x77, 0xc0];
/// NERDS blue darkened for text on a light background (6:1 on the light page).
pub const NERDS_INK_LIGHT: [u8; 3] = [0x00, 0x5e, 0x99];
/// NERDS blue lightened for text on a dark background (8:1 on the dark page).
pub const NERDS_INK_DARK: [u8; 3] = [0x5c, 0xb6, 0xf2];

/// The suite, in display order. `name` is what each upstream build installs as; `display` is the
/// NERDS name shown everywhere in the launcher.
pub const SUITE: [CraftApp; 13] = [
    app("photocraft", "PhotoCraft", "NERDS Photo", "Image editing", "storytold/photocraft"),
    app("vectorcraft", "VectorCraft", "NERDS Vector", "Vector illustration", "storytold/vectorcraft"),
    app("filmcraft", "FilmCraft", "NERDS Film", "Video editing, color and sound", "storytold/filmcraft"),
    app("lightcraft", "LightCraft", "NERDS Light", "Photo library and raw development", "storytold/lightcraft"),
    CraftApp {
        former_ids: &["printcraft"],
        former_names: &["PrintCraft"],
        ..app("pdfcraft", "PdfCraft", "NERDS PDF", "Reading, organizing and protecting PDFs", "storytold/pdfcraft")
    },
    app("effectcraft", "EffectCraft", "NERDS Effects", "Motion graphics and visual effects", "storytold/effectcraft"),
    app("designcraft", "DesignCraft", "NERDS Layout", "Page layout and publishing", "storytold/designcraft"),
    app("figcraft", "FigCraft", "NERDS Proto", "Interface design and prototyping", "storytold/figcraft"),
    app("wordcraft", "WordCraft", "NERDS Word", "Writing and document design", "storytold/wordcraft"),
    app("soundcraft", "SoundCraft", "NERDS Sound", "Recording, editing and mixing audio", "storytold/soundcraft"),
    app("deckcraft", "DeckCraft", "NERDS Deck", "Presentations and slide shows", "storytold/deckcraft"),
    app("cadcraft", "CADCraft", "NERDS CAD", "Computer-aided design and drafting", "storytold/cadcraft"),
    app("gridcraft", "GridCraft", "NERDS Grid", "Spreadsheets and data", "storytold/gridcraft"),
];

// NERDS links. An empty URL hides that link everywhere (footer, Settings, tray menu); fill these
// in once NERDS has them.

/// The NERDS website.
pub const WEBSITE_URL: &str = "";
/// The GitHub account every app's repository lives under: the only one the launcher
/// installs from, whatever the release feed says.
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
pub const LAUNCHER: CraftApp =
    app("artcraft-launcher", "NERDS Launcher", "NERDS Launcher", "Installs and updates the NERDS Suite", "nerds-coach/nerds-launcher");

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
            assert_eq!(a.repo, format!("storytold/{}", a.id));
            assert_eq!(a.name.to_lowercase(), a.id);
            assert!(a.display.starts_with("NERDS "), "{} has no NERDS name", a.id);
            assert_eq!((a.color, a.ink_light, a.ink_dark), (NERDS_BLUE, NERDS_INK_LIGHT, NERDS_INK_DARK));
        }
        let mut shown: Vec<_> = SUITE.iter().map(|a| a.display).collect();
        shown.sort_unstable();
        shown.dedup();
        assert_eq!(shown.len(), SUITE.len(), "two apps share a NERDS name");
    }

    #[test]
    fn pdfcraft_also_answers_to_printcraft() {
        let pdf = CraftApp::by_id("pdfcraft").unwrap();
        assert_eq!(pdf.stems().collect::<Vec<_>>(), ["pdfcraft", "printcraft"]);
        assert_eq!(pdf.names().collect::<Vec<_>>(), ["PdfCraft", "PrintCraft"]);
        assert_eq!(pdf.website(), "https://github.com/storytold/pdfcraft");
        assert_eq!(CraftApp::by_id("wordcraft").unwrap().website(), "https://github.com/storytold/wordcraft");
    }
}
