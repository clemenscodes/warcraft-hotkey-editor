mod props;
mod style;

use dioxus::prelude::*;
pub use props::UnitCardIconProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitCardIcon);

/// The portrait thumbnail of a unit card, or an empty framed square when the unit
/// has no icon.
#[component]
pub fn UnitCardIcon(props: UnitCardIconProps) -> Element {
    let icon_path = props.icon_path;
    let display_name = props.display_name;
    let icon_url = icon_path.map(|url| url.to_string());
    let Some(source) = icon_url else {
        return rsx! {
            div { class: CLASS }
        };
    };
    rsx! {
        img {
            class: CLASS,
            src: source,
            alt: display_name,
            loading: "lazy",
            decoding: "async",
        }
    }
}
