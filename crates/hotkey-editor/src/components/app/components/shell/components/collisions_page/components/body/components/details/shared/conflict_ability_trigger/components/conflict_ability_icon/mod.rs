mod model;
mod view;

pub use view::ConflictAbilityIconView;
mod style;
use crate::components::app::components::shell::components::shared::framed_icon::{
    FramedIcon, IconRadius,
};
use dioxus::prelude::*;
use model::ConflictAbilityIconModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ConflictAbilityIcon(props: ConflictAbilityIconModel) -> Element {
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
