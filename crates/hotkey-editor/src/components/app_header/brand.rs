use dioxus::prelude::*;

const HEADER_GOLD_DECORATION: Asset = asset!("/assets/webui/common/header-decoration-gold.png");

#[component]
pub(crate) fn AppHeaderBrand() -> Element {
    rsx! {
        div { class: "app-header-brand",
            img {
                class: "wc3-header-decoration",
                src: "{HEADER_GOLD_DECORATION}",
                alt: "",
                aria_hidden: "true",
            }
            h1 { class: "app-header-title", "Warcraft III Hotkey Editor" }
            img {
                class: "wc3-header-decoration wc3-header-decoration-mirrored",
                src: "{HEADER_GOLD_DECORATION}",
                alt: "",
                aria_hidden: "true",
            }
        }
    }
}
