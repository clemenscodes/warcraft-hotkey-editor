use dioxus::prelude::*;
use warcraft_api::{UnitKind, UnitKindHelpers};

/// The tab's kind together with the active-category signal it flips on tap, read from
/// editor context by the component and handed to the model builder.
pub(super) struct MobileCategoryTabInputs {
    pub(super) kind: UnitKind,
    pub(super) active_category: Signal<UnitKind>,
}

/// A category tab's shaped view: its display label and the tap handler that makes its
/// kind the active category.
pub(super) struct MobileCategoryTabModel {
    label: &'static str,
    onclick: EventHandler<MouseEvent>,
}

impl From<MobileCategoryTabInputs> for MobileCategoryTabModel {
    fn from(inputs: MobileCategoryTabInputs) -> Self {
        let kind = inputs.kind;
        let label = UnitKindHelpers::category_label(kind);
        let mut active_category = inputs.active_category;
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            active_category.set(kind);
        });
        Self { label, onclick }
    }
}

impl MobileCategoryTabModel {
    pub(super) fn label(&self) -> &'static str {
        self.label
    }

    pub(super) fn onclick(&self) -> EventHandler<MouseEvent> {
        self.onclick
    }
}
