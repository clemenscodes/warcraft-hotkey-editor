mod props;
mod style;
use crate::components::app::components::shell::components::shared::framed_icon::{
    FramedIcon, FramedIconProps, IconRadius,
};
use dioxus::prelude::*;
pub use props::ConflictUnitIconProps;
use style::CLASS;
use tw_macro::assert_component;

/// A conflicting unit's portrait. It owns its slot (a hover `.group`) and lifts to a
/// gold glow on hover; the shared `FramedIcon` draws the bordered, rounded image and
/// renders nothing when the unit has no icon.
#[component]
pub fn ConflictUnitIcon(props: ConflictUnitIconProps) -> Element {
    let Some(source) = props.src else {
        return rsx! {};
    };
    let src = Some(source);
    let alt = props.alt;
    let framed = FramedIconProps {
        src,
        alt,
        radius: IconRadius::Tile,
        hover_glow: true,
        placeholder: false,
    };
    rsx! {
        div {
            class: CLASS,
            FramedIcon { ..framed }
        }
    }
}

assert_component!(ConflictUnitIcon);
