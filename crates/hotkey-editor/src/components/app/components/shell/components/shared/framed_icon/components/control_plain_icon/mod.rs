mod model;
mod view;

pub use view::ControlPlainIconView;
mod style;

use crate::components::app::components::shell::components::shared::framed_icon::components::shared::framed_icon_image::FramedIconImage;
use dioxus::prelude::*;
use model::ControlPlainIconModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ControlPlainIcon(props: ControlPlainIconModel) -> Element {
    let Some(source) = props.source else {
        return rsx! {
            div {
                class: CLASS,
            }
        };
    };
    let alt = props.alt;
    rsx! {
        div {
            class: CLASS,
            FramedIconImage {
                source,
                alt,
            }
        }
    }
}

assert_component!(ControlPlainIcon);
