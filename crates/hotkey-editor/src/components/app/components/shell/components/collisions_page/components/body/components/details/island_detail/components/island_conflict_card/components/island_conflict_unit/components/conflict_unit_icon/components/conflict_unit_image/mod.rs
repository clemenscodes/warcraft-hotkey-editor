mod props;
mod style;
use dioxus::prelude::*;
pub use props::ConflictUnitImageProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictUnitImage);
/// The conflicting unit's portrait image. It fills the icon slot its host hands
/// it, scaling with that slot rather than pinning its own size.
#[component]
pub fn ConflictUnitImage(props: ConflictUnitImageProps) -> Element {
    let source = props.source;
    let alt = props.alt;
    rsx! {
        img {
            class: CLASS,
            src: source,
            alt,
            loading: "lazy",
            decoding: "async",
        }
    }
}
