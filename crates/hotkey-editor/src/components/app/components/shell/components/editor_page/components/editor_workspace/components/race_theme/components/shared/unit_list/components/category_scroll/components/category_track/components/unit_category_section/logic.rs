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

/// The heading's shaped data: its label, collapsed state, and the toggle handler the
/// section hands down to `UnitCategoryHeading` as named fields.
pub(super) struct CategoryHeadingData {
    pub(super) label: String,
    pub(super) is_collapsed: bool,
    pub(super) on_toggle: EventHandler<MouseEvent>,
}

impl From<UnitCategoryHeadingInputs> for CategoryHeadingData {
    fn from(inputs: UnitCategoryHeadingInputs) -> Self {
        let category_kind = inputs.category_kind;
        let label = UnitKindHelpers::category_label(category_kind).to_owned();
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
            is_collapsed,
            on_toggle,
        }
    }
}
