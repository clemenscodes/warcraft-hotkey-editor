pub mod components;
mod model;
mod view;

pub use view::ConflictUnitIconHostView;
mod style;

use components::conflict_unit_icon::ConflictUnitIcon;
use dioxus::prelude::*;
use model::ConflictUnitIconHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ConflictUnitIconHost(props: ConflictUnitIconHostModel) -> Element {
    let Some(source) = props.src else {
        return rsx! {};
    };
    let src = Some(source);
    let alt = props.alt;
    rsx! {
        div {
            class: CLASS,
            ConflictUnitIcon {
                src,
                alt,
            }
        }
    }
}

assert_component!(ConflictUnitIconHost);
