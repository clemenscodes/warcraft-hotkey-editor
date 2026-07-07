mod props;
mod style;
use dioxus::prelude::*;
pub use props::HotkeyUnitRowIconProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HotkeyUnitRowIcon);
/// A unit's portrait on a collision card. A guarded leaf: renders nothing when
/// the unit has no icon.
#[component]
pub fn HotkeyUnitRowIcon(props: HotkeyUnitRowIconProps) -> Element {
    let Some(src) = props.icon_url else {
        return rsx! {};
    };
    let alt = props.alt;
    rsx! {
        img {
            class: CLASS,
            src,
            alt,
            loading: "lazy",
            decoding: "async"
        }
    }
}
