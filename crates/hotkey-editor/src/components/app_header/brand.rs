use dioxus::prelude::*;

const HEADER_GOLD_DECORATION: Asset = asset!("/assets/webui/common/header-decoration-gold.png");

#[component]
pub(crate) fn AppHeaderBrand() -> Element {
    rsx! {
        div { class: "app-header-brand",
            img {
                class: "h-[2.4rem] w-auto flex-none [filter:drop-shadow(0_1px_0_rgba(0,0,0,0.7))] max-[1099px]:hidden",
                src: "{HEADER_GOLD_DECORATION}",
                alt: "",
                aria_hidden: "true",
            }
            h1 { class: "app-header-title", "Warcraft III Hotkey Editor" }
            img {
                class: "h-[2.4rem] w-auto flex-none [filter:drop-shadow(0_1px_0_rgba(0,0,0,0.7))] [transform:scaleX(-1)] max-[1099px]:hidden",
                src: "{HEADER_GOLD_DECORATION}",
                alt: "",
                aria_hidden: "true",
            }
        }
    }
}
