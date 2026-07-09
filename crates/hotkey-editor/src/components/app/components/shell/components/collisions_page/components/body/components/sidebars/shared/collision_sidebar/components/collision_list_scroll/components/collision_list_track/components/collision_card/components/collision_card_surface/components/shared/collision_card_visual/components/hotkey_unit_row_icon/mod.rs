mod props;
mod style;

use crate::components::app::components::shell::components::shared::framed_icon::{
    FramedIcon, FramedIconProps, IconRadius,
};
use dioxus::prelude::*;
pub use props::HotkeyUnitRowIconProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HotkeyUnitRowIcon);

/// A unit's portrait on a collision card. It owns its per-band slot; the shared
/// `FramedIcon` draws the bordered, rounded image and renders nothing when the unit
/// has no icon.
#[component]
pub fn HotkeyUnitRowIcon(props: HotkeyUnitRowIconProps) -> Element {
    let Some(source) = props.icon_url else {
        return rsx! {};
    };
    let src = Some(source);
    let alt = props.alt;
    let framed = FramedIconProps {
        src,
        alt,
        radius: IconRadius::Control,
        hover_glow: false,
        placeholder: false,
    };
    rsx! {
        div {
            class: CLASS,
            FramedIcon { ..framed }
        }
    }
}
