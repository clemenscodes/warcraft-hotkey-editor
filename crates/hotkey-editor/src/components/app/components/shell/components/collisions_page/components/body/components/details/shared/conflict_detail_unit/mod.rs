pub mod components;
mod model;
mod presentation;
mod view;

pub use view::ConflictDetailUnitView;
mod style;
use components::conflict_detail_unit_icon::ConflictDetailUnitIcon;
use dioxus::prelude::*;
use model::ConflictDetailUnitModel;
use presentation::{ConflictDetailUnitPresentation, use_conflict_detail_unit};
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn ConflictDetailUnit(props: ConflictDetailUnitModel) -> Element {
    let ConflictDetailUnitPresentation {
        icon_src,
        icon_alt,
        onclick,
    } = use_conflict_detail_unit(&props);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            ConflictDetailUnitIcon {
                src: icon_src,
                alt: icon_alt,
            }
        }
    }
}

assert_component!(ConflictDetailUnit);
