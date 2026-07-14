use dioxus::prelude::*;
use tw_macro::assert_component;

const OG_DESCRIPTION: &str = "Visual command-card editor for Warcraft III: Reforged. Drag keys, export CustomKeys.txt — runs entirely in your browser.";
const OG_IMAGE: &str = "https://clemenscodes.github.io/warcraft-hotkey-editor/og-image.png";
const OG_URL: &str = "https://clemenscodes.github.io/warcraft-hotkey-editor/";
const TAILWIND_STYLES: Asset = asset!("/assets/tailwind.css");
const KEYBOARD_NAVIGATION_SCRIPT: Asset = asset!("/assets/keyboard-navigation.js");
const FAVICON: Asset = asset!("/assets/favicon.svg");

#[component]
pub fn Head() -> Element {
    rsx! {
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1",
        }
        document::Meta {
            name: "theme-color",
            content: "#050a1a",
        }
        document::Meta {
            name: "twitter:card",
            content: "summary_large_image",
        }
        document::Meta {
            property: "og:type",
            content: "website",
        }
        document::Meta {
            property: "og:title",
            content: "Warcraft III Hotkey Editor",
        }
        document::Meta {
            property: "og:description",
            content: OG_DESCRIPTION,
        }
        document::Meta {
            property: "og:image",
            content: OG_IMAGE,
        }
        document::Meta {
            property: "og:url",
            content: OG_URL,
        }
        document::Link {
            rel: "preload",
            href: "/warcraft-hotkey-editor/fonts/frizqt.ttf",
            r#as: "font",
            r#type: "font/ttf",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "apple-touch-icon",
            href: "icon-192.png",
        }
        document::Link {
            rel: "icon",
            r#type: "image/x-icon",
            href: "favicon.ico",
        }
        document::Link {
            rel: "icon",
            r#type: "image/svg+xml",
            href: FAVICON,
        }
        document::Stylesheet {
            href: TAILWIND_STYLES,
        }
        document::Script {
            src: KEYBOARD_NAVIGATION_SCRIPT,
            r#type: "module",
        }
    }
}

assert_component!(Head);
