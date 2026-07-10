mod props;
mod view;

pub use view::HotkeyUnitRowIconView;
mod style;

use crate::components::app::components::shell::components::shared::framed_icon::{
    FramedIcon, IconRadius,
};
use dioxus::prelude::*;
use props::HotkeyUnitRowIconProps;
use style::CLASS;
use tw_macro::assert_component;

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
    let radius = IconRadius::Control;
    rsx! {
        div {
            class: CLASS,
            FramedIcon {
                src,
                alt,
                radius,
                hover_glow: false,
                placeholder: false,
            }
        }
    }
}

assert_component!(HotkeyUnitRowIcon);
