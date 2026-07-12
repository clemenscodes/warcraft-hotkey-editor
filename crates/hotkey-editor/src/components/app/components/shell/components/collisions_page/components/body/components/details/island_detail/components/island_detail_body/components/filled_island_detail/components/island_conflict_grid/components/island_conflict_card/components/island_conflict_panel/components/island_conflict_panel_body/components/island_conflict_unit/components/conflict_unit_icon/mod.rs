mod model;
mod view;

pub use view::ConflictUnitIconView;
mod style;
use crate::components::app::components::shell::components::shared::framed_icon::{
    FramedIcon, IconRadius,
};
use dioxus::prelude::*;
use model::ConflictUnitIconModel;
use style::CLASS;
use tw_macro::assert_component;

/// A conflicting unit's portrait. It owns its slot (a hover `.group`) and lifts to a
/// gold glow on hover; the shared `FramedIcon` draws the bordered, rounded image and
/// renders nothing when the unit has no icon.
#[component]
pub fn ConflictUnitIcon(props: ConflictUnitIconModel) -> Element {
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

assert_component!(ConflictUnitIcon);
