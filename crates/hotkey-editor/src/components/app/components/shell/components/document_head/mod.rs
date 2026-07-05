use crate::components::app::{FAVICON, KEYBOARD_NAVIGATION_SCRIPT, TAILWIND_STYLES};
use dioxus::prelude::*;

const OG_DESCRIPTION: &str = "Visual command-card editor for Warcraft III: Reforged. \
                              Drag keys, export CustomKeys.txt — runs entirely in your browser.";
const OG_IMAGE: &str = "https://clemenscodes.github.io/warcraft-hotkey-editor/og-image.png";
const OG_URL: &str = "https://clemenscodes.github.io/warcraft-hotkey-editor/";

/// The page's `<head>`: the stylesheet, the keyboard-navigation module, the icons,
/// the viewport, and the social / OpenGraph tags — all hoisted into the document head
/// by Dioxus. Split out of `Workbench` so that component stays the editor's layout,
/// not a metadata manifest.
///
/// The viewport is intentionally *not* `viewport-fit=cover`: the browser keeps the whole
/// app inside the safe area, so no shell component has to re-inset itself with
/// `env(safe-area-inset-*)`. `theme-color` paints the browser chrome (and, with the `html`
/// background, the safe-area strips on a notched phone) the app's own dark blue.
#[component]
pub fn DocumentHead() -> Element {
    rsx! {
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1",
        }
        document::Meta {
            name: "theme-color",
            content: "#050a1a"
        }
        document::Meta {
            property: "og:type",
            content: "website"
        }
        document::Meta {
            property: "og:title",
            content: "Warcraft III Hotkey Editor"
        }
        document::Meta {
            property: "og:description",
            content: OG_DESCRIPTION
        }
        document::Meta {
            property: "og:image",
            content: OG_IMAGE
        }
        document::Meta {
            property: "og:url",
            content: OG_URL
        }
        document::Meta {
            name: "twitter:card",
            content: "summary_large_image"
        }
        document::Stylesheet {
            href: TAILWIND_STYLES
        }
        document::Script {
            src: KEYBOARD_NAVIGATION_SCRIPT,
            r#type: "module",
        }
        document::Link {
            rel: "icon",
            r#type: "image/svg+xml",
            href: FAVICON,
        }
        document::Link {
            rel: "icon",
            r#type: "image/x-icon",
            href: "favicon.ico",
        }
        document::Link {
            rel: "apple-touch-icon",
            href: "icon-192.png"
        }
    }
}
