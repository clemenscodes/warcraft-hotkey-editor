mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::UnitCardIconProps;

assert_component!(UnitCardIcon);

/// The portrait thumbnail of a unit card, or an empty framed square when the unit
/// has no icon.
#[component]
pub fn UnitCardIcon(props: UnitCardIconProps) -> Element {
    let icon_path = props.icon_path;
    let display_name = props.display_name;
    let icon_url = icon_path.map(|url| url.to_string());
    rsx! {
        if let Some(source) = icon_url {
            img {
                class: CLASS,
                src: source,
                alt: display_name,
                loading: "lazy",
                decoding: "async",
            }
        } else {
            div { class: CLASS }
        }
    }
}
