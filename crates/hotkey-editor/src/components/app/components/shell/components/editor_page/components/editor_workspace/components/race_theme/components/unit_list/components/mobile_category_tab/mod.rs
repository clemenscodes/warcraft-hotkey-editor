mod props;
mod style;

use tw_macro::assert_component;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_list::unit_kind_data_attr;
use dioxus::prelude::*;
pub use props::MobileCategoryTabProps;
use warcraft_database::UnitKindHelpers;
assert_component!(MobileCategoryTab);

/// A single category tab in the mobile unit picker.
#[component]
pub fn MobileCategoryTab(props: MobileCategoryTabProps) -> Element {
    let kind = props.kind;
    let is_active = props.is_active;
    let class = style::class(props.race);
    let mut active_category = props.active_category;
    let label = UnitKindHelpers::category_label(kind);
    let kind_attr = unit_kind_data_attr(kind);
    let handle_click = move |_| active_category.set(kind);
    rsx! {
        button {
            class,
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
