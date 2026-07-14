pub mod components;
mod model;
mod view;

pub use view::UnitCategoryHeadingView;
mod style;

use components::category_chevron::CategoryChevron;
use dioxus::prelude::*;
use model::UnitCategoryHeadingModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitCategoryHeading(props: UnitCategoryHeadingModel) -> Element {
    let is_collapsed = props.is_collapsed;
    let label = props.label;
    let on_toggle = props.on_toggle;
    rsx! {
        button {
            class: CLASS,
            onclick: on_toggle,
            CategoryChevron {
                is_collapsed,
            }
            {label}
        }
    }
}

assert_component!(UnitCategoryHeading);
