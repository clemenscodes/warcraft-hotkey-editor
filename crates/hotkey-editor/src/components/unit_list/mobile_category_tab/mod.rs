mod props;
mod style;

use dioxus::prelude::*;
use warcraft_database::UnitKindHelpers;

use crate::assert_component;
use style::CLASS;

use super::unit_kind_data_attr;

pub use props::MobileCategoryTabProps;

assert_component!(MobileCategoryTab);

/// A single category tab in the mobile unit picker.
#[component]
pub fn MobileCategoryTab(props: MobileCategoryTabProps) -> Element {
    let kind = props.kind;
    let is_active = props.is_active;
    let mut active_category = props.active_category;
    let label = UnitKindHelpers::category_label(kind);
    let kind_attr = unit_kind_data_attr(kind);
    let handle_click = move |_| active_category.set(kind);
    rsx! {
        button {
            class: CLASS,
            role: "tab",
            r#type: "button",
            aria_selected: is_active,
            "data-unit-kind": kind_attr,
            "data-active": is_active,
            onclick: handle_click,
            {label}
        }
    }
}
