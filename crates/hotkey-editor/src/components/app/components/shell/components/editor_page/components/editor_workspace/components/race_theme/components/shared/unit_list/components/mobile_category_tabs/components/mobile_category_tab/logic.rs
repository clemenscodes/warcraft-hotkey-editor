use super::components::active_mobile_category_tab::ActiveMobileCategoryTabProps;
use super::components::idle_mobile_category_tab::IdleMobileCategoryTabProps;
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

impl From<&MobileCategoryTabModel> for ActiveMobileCategoryTabProps {
    fn from(model: &MobileCategoryTabModel) -> Self {
        let label = model.label;
        let onclick = model.onclick;
        Self { label, onclick }
    }
}

impl From<&MobileCategoryTabModel> for IdleMobileCategoryTabProps {
    fn from(model: &MobileCategoryTabModel) -> Self {
        let label = model.label;
        let onclick = model.onclick;
        Self { label, onclick }
    }
}
