use dioxus::prelude::*;

const HEADER_GOLD_DECORATION: Asset = asset!("/assets/webui/common/header-decoration-gold.png");

#[component]
pub(crate) fn DialogHeader(title: String, on_close: EventHandler<()>) -> Element {
    rsx! {
        header { class: "wc3-dialog-header",
            img {
                class: "wc3-header-decoration",
                src: "{HEADER_GOLD_DECORATION}",
                alt: "",
                aria_hidden: "true",
            }
            h2 { class: "wc3-dialog-title", "{title}" }
            img {
                class: "wc3-header-decoration wc3-header-decoration-mirrored",
                src: "{HEADER_GOLD_DECORATION}",
                alt: "",
                aria_hidden: "true",
            }
            button {
                class: "wc3-close-button",
                r#type: "button",
                aria_label: "Close",
                onclick: move |_| on_close.call(()),
                "\u{2715}"
            }
        }
    }
}
