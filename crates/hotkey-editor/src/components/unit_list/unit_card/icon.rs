use dioxus::prelude::*;

use crate::model::icons::IconUrl;

#[component]
pub(super) fn UnitCardIcon(icon_path: Option<IconUrl>, display_name: String) -> Element {
    rsx! {
        if let Some(source) = icon_path {
            img {
                class: "w-20 h-20 border border-warcraft-blue rounded-[3px] shrink-0 object-cover \
                        bg-[rgba(20,35,60,0.7)] text-transparent",
                src: "{source}",
                alt: display_name,
                loading: "lazy",
                decoding: "async",
            }
        } else {
            div { class: "w-20 h-20 border border-warcraft-blue rounded-[3px] shrink-0 bg-[rgba(20,35,60,0.7)]" }
        }
    }
}
