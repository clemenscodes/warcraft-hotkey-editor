mod model;
mod view;

pub use view::HotkeyUnitRowIconView;
mod style;

use crate::components::app::components::shell::components::shared::framed_icon::{
    FramedIcon, IconRadius,
};
use dioxus::prelude::*;
use model::HotkeyUnitRowIconModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HotkeyUnitRowIcon(props: HotkeyUnitRowIconModel) -> Element {
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
