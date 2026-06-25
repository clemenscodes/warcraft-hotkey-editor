use dioxus::prelude::*;
use warcraft_api::UnitKind;
use warcraft_database::UnitKindHelpers;

use super::unit_kind_data_attr;

#[derive(Props, Clone, PartialEq)]
pub struct MobileCategoryTabProps {
    pub kind: UnitKind,
    pub is_active: bool,
    pub active_category: Signal<UnitKind>,
}

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
            class: if is_active { "unit-category-tab active" } else { "unit-category-tab" },
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
