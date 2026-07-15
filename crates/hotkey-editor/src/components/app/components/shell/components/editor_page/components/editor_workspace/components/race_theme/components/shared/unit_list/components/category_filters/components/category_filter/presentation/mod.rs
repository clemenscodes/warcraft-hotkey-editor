use dioxus::prelude::*;
use std::collections::HashSet;
use warcraft_api::UnitKind;

pub(super) struct CategoryFilterInputs {
    pub(super) kind: UnitKind,
    pub(super) collapsed_categories: Signal<HashSet<UnitKind>>,
}

pub(super) struct CategoryFilterPresentation {
    label: &'static str,
    onclick: EventHandler<MouseEvent>,
}

impl From<CategoryFilterInputs> for CategoryFilterPresentation {
    fn from(inputs: CategoryFilterInputs) -> Self {
        let kind = inputs.kind;
        let label = kind.category_label();
        let mut collapsed_categories = inputs.collapsed_categories;
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            let mut categories = collapsed_categories.write();
            if categories.contains(&kind) {
                categories.remove(&kind);
            } else {
                categories.insert(kind);
            }
        });
        Self { label, onclick }
    }
}

impl CategoryFilterPresentation {
    pub(super) fn label(&self) -> &'static str {
        self.label
    }

    pub(super) fn onclick(&self) -> EventHandler<MouseEvent> {
        self.onclick
    }
}
use crate::services::editor_state::context::use_editor_state;

pub(super) struct CategoryFilterDispatch {
    pub(super) is_active: bool,
    pub(super) model: CategoryFilterPresentation,
}

pub(super) fn use_category_filter(kind: UnitKind) -> CategoryFilterDispatch {
    let collapsed_categories = use_editor_state().collapsed_categories();
    let is_active = !collapsed_categories.read().contains(&kind);
    let inputs = CategoryFilterInputs {
        kind,
        collapsed_categories,
    };
    let model = CategoryFilterPresentation::from(inputs);
    CategoryFilterDispatch { is_active, model }
}
