pub mod components;
mod model;
mod view;

pub use view::UnitPortraitHostView;
mod style;

use components::unit_portrait::UnitPortrait;
use dioxus::prelude::*;
use model::UnitPortraitHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitPortraitHost(props: UnitPortraitHostModel) -> Element {
    let Some(src) = props.src else {
        return rsx! {};
    };
    let alt = props.alt;
    rsx! {
        div {
            class: CLASS,
            UnitPortrait {
                src,
                alt,
            }
        }
    }
}

assert_component!(UnitPortraitHost);
