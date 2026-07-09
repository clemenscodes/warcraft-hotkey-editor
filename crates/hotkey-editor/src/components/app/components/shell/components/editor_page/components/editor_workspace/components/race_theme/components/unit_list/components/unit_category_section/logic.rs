use super::components::unit_category_heading::UnitCategoryHeadingProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_list::unit_kind_data_attr;
use dioxus::prelude::*;
use std::collections::HashSet;
use warcraft_api::{UnitKind, UnitKindHelpers};

/// The heading's inputs: which category it heads, whether it is collapsed, and the
/// collapsed-set signal its toggle flips. The section reads the collapsed set from
/// context and hands these to the heading builder.
pub(super) struct UnitCategoryHeadingInputs {
    pub(super) category_kind: UnitKind,
    pub(super) is_collapsed: bool,
    pub(super) collapsed_categories: Signal<HashSet<UnitKind>>,
}

impl From<UnitCategoryHeadingInputs> for UnitCategoryHeadingProps {
    fn from(inputs: UnitCategoryHeadingInputs) -> Self {
        let category_kind = inputs.category_kind;
        let label = UnitKindHelpers::category_label(category_kind).to_owned();
        let kind_attr = unit_kind_data_attr(category_kind);
        let is_collapsed = inputs.is_collapsed;
        let mut collapsed_categories = inputs.collapsed_categories;
        let on_toggle = EventHandler::new(move |_event: MouseEvent| {
            let mut categories = collapsed_categories.write();
            if categories.contains(&category_kind) {
                categories.remove(&category_kind);
            } else {
                categories.insert(category_kind);
            }
        });
        Self {
            label,
            kind_attr,
            is_collapsed,
            on_toggle,
        }
    }
}
