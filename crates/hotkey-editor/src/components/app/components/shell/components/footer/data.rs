const HEART_SVG: &str = r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true" focusable="false"><path fill="currentColor" d="M12 21s-7.5-4.6-9.9-9.3C.3 7.9 2.7 4 6.4 4c2 0 3.6 1 4.6 2.3h2C14 5 15.6 4 17.6 4c3.7 0 6.1 3.9 4.3 7.7C19.5 16.4 12 21 12 21Z"/></svg>"##;
const GITHUB_SVG: &str = r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true" focusable="false"><path fill="currentColor" d="M12 .5C5.65.5.5 5.65.5 12c0 5.08 3.29 9.39 7.86 10.91.58.11.79-.25.79-.56 0-.27-.01-1-.02-1.96-3.2.7-3.87-1.54-3.87-1.54-.52-1.32-1.27-1.68-1.27-1.68-1.04-.71.08-.7.08-.7 1.15.08 1.76 1.18 1.76 1.18 1.02 1.75 2.69 1.24 3.35.95.1-.74.4-1.24.73-1.53-2.55-.29-5.24-1.28-5.24-5.69 0-1.26.45-2.28 1.18-3.08-.12-.29-.51-1.46.11-3.05 0 0 .97-.31 3.18 1.18a11.07 11.07 0 0 1 5.78 0c2.21-1.49 3.18-1.18 3.18-1.18.62 1.59.23 2.76.11 3.05.74.8 1.18 1.82 1.18 3.08 0 4.42-2.69 5.39-5.25 5.68.41.36.78 1.06.78 2.13 0 1.54-.01 2.78-.01 3.16 0 .31.21.68.8.56C20.21 21.39 23.5 17.08 23.5 12 23.5 5.65 18.35.5 12 .5Z"/></svg>"##;
const REPO_URL: &str = "https://github.com/clemenscodes/warcraft-hotkey-editor";

const LICENSE_URL: &str =
    "https://github.com/clemenscodes/warcraft-hotkey-editor/blob/main/LICENSE";

const DISCLAIMER_URL: &str =
    "https://github.com/clemenscodes/warcraft-hotkey-editor/blob/main/DISCLAIMER.md";

/// The authorship line's content: the two text fragments that flank the heart
/// glyph. Sourced here and threaded into `FooterCredit` as named fields.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) struct FooterCreditContent {
    pub(super) lead: &'static str,
    pub(super) tail: &'static str,
    pub(super) heart: &'static str,
}

pub(super) const CREDIT: FooterCreditContent = FooterCreditContent {
    lead: "Crafted with",
    tail: "by Clemens",
    heart: HEART_SVG,
};

/// One outbound footer link: its visible label, destination, and optional inline
/// glyph. The GitHub entry carries a glyph; the others are plain text links
/// (`icon` is `None`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) struct FooterLinkContent {
    pub(super) label: &'static str,
    pub(super) href: &'static str,
    pub(super) icon: Option<&'static str>,
}

/// The outbound links, looped over by the footer.
pub(super) const LINKS: &[FooterLinkContent] = &[
    FooterLinkContent {
        label: "Source on GitHub",
        href: REPO_URL,
        icon: Some(GITHUB_SVG),
    },
    FooterLinkContent {
        label: "AGPL-3.0",
        href: LICENSE_URL,
        icon: None,
    },
    FooterLinkContent {
        label: "Disclaimer",
        href: DISCLAIMER_URL,
        icon: None,
    },
];

/// The trademark disclaimer, sourced here and threaded into `FooterDisclaimer`.
pub(super) const DISCLAIMER: &str = "Not affiliated with or endorsed by Blizzard Entertainment. \
     Warcraft III is a trademark of Blizzard Entertainment, Inc.";
