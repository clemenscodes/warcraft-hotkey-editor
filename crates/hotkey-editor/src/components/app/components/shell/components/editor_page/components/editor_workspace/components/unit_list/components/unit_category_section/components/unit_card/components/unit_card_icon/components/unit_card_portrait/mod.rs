mod props;
mod style;

use dioxus::prelude::*;
pub use props::UnitCardPortraitProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitCardPortrait);

/// The portrait thumbnail image. It fills the icon slot its host hands it, scaling
/// with that slot rather than pinning its own size.
#[component]
pub fn UnitCardPortrait(props: UnitCardPortraitProps) -> Element {
    let source = props.source;
    let display_name = props.display_name;
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
