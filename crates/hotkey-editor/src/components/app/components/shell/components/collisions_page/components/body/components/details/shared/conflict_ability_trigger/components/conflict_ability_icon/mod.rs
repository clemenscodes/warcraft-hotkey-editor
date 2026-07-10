mod props;
mod style;
use crate::components::app::components::shell::components::shared::framed_icon::{
    FramedIcon, IconRadius,
};
use dioxus::prelude::*;
use props::ConflictAbilityIconProps;
use style::CLASS;
use tw_macro::assert_component;

/// One conflicting ability's square icon: it owns its slot and lifts to a gold glow
/// when its trigger `.group` is hovered. The shared `FramedIcon` draws the bordered,
/// rounded image.
#[component]
pub fn ConflictAbilityIcon(props: ConflictAbilityIconProps) -> Element {
    let Some(source) = props.src else {
        return rsx! {};
    };
    let src = Some(source);
    let alt = props.alt;
    let radius = IconRadius::Tile;
    rsx! {
        div {
            class: CLASS,
            FramedIcon {
                src,
                alt,
                radius,
                hover_glow: true,
                placeholder: false,
            }
        }
    }
}

assert_component!(ConflictAbilityIcon);
