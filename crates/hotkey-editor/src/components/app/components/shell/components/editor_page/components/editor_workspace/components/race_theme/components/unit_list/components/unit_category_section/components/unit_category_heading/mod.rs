pub mod components;
mod props;
mod style;

use components::category_chevron::{CategoryChevron, CategoryChevronProps};
use dioxus::prelude::*;
pub use props::UnitCategoryHeadingProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitCategoryHeading);

/// The clickable heading for a unit category in the sidebar list.
#[component]
pub fn UnitCategoryHeading(props: UnitCategoryHeadingProps) -> Element {
    let chevron = CategoryChevronProps::from(&props);
    let label = props.label;
    let kind_attr = props.kind_attr;
    let is_collapsed = props.is_collapsed;
    let on_toggle = props.on_toggle;
    rsx! {
        button {
            class: CLASS,
            "data-unit-kind": kind_attr,
            "data-collapsed": is_collapsed,
            onclick: on_toggle,
            CategoryChevron { ..chevron }
            {label}
        }
    }
}
