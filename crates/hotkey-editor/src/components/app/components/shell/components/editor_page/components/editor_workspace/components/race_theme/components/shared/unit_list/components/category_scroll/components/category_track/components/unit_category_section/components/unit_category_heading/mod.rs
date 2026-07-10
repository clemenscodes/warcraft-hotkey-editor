pub mod components;
mod props;
mod style;

use components::category_chevron::{CategoryChevron, CategoryChevronProps};
use dioxus::prelude::*;
pub use props::UnitCategoryHeadingProps;
use style::CLASS;
use tw_macro::assert_component;

/// The clickable heading for a unit category in the sidebar list.
#[component]
pub fn UnitCategoryHeading(props: UnitCategoryHeadingProps) -> Element {
    let chevron = CategoryChevronProps::from(&props);
    let label = props.label;
    let on_toggle = props.on_toggle;
    rsx! {
        button {
            class: CLASS,
            onclick: on_toggle,
            CategoryChevron { ..chevron }
            {label}
        }
    }
}

assert_component!(UnitCategoryHeading);
