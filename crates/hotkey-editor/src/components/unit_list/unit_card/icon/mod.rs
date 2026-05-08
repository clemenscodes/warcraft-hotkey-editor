use dioxus::prelude::*;

use crate::model::icons::IconUrl;

#[derive(Props, Clone, PartialEq)]
pub(super) struct UnitCardIconProps {
    pub(super) icon_path: Option<IconUrl>,
    pub(super) display_name: String,
}

#[component]
pub(super) fn UnitCardIcon(props: UnitCardIconProps) -> Element {
    let icon_path = props.icon_path;
    let display_name = props.display_name;
    let icon_url = icon_path.map(|url| url.to_string());
    rsx! {
        if let Some(source) = icon_url {
            img {
                class: "unit-card-icon",
                src: source,
                alt: display_name,
                loading: "lazy",
                decoding: "async",
            }
        } else {
            div { class: "unit-card-icon" }
        }
    }
}
