use super::components::active_mobile_category_tab::ActiveMobileCategoryTabProps;
use super::components::idle_mobile_category_tab::IdleMobileCategoryTabProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_list::unit_kind_data_attr;
use dioxus::prelude::*;
use warcraft_api::{UnitKind, UnitKindHelpers};

/// The tab's kind together with the active-category signal it flips on tap, read from
/// editor context by the component and handed to the model builder.
pub(super) struct MobileCategoryTabInputs {
    pub(super) kind: UnitKind,
    pub(super) active_category: Signal<UnitKind>,
}

/// A category tab's shaped view: its display label, its kind's data attribute (the card
/// filter reads it), and the tap handler that makes its kind the active category.
pub(super) struct MobileCategoryTabModel {
    label: &'static str,
    kind_attr: &'static str,
    onclick: EventHandler<MouseEvent>,
}

impl From<MobileCategoryTabInputs> for MobileCategoryTabModel {
    fn from(inputs: MobileCategoryTabInputs) -> Self {
        let kind = inputs.kind;
        let label = UnitKindHelpers::category_label(kind);
        let kind_attr = unit_kind_data_attr(kind);
        let mut active_category = inputs.active_category;
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            active_category.set(kind);
        });
        Self {
            label,
            kind_attr,
            onclick,
        }
    }
}

impl From<&MobileCategoryTabModel> for ActiveMobileCategoryTabProps {
    fn from(model: &MobileCategoryTabModel) -> Self {
        let label = model.label;
        let kind_attr = model.kind_attr;
        let onclick = model.onclick;
        Self {
            label,
            kind_attr,
            onclick,
        }
    }
}

impl From<&MobileCategoryTabModel> for IdleMobileCategoryTabProps {
    fn from(model: &MobileCategoryTabModel) -> Self {
        let label = model.label;
        let kind_attr = model.kind_attr;
        let onclick = model.onclick;
        Self {
            label,
            kind_attr,
            onclick,
        }
    }
}
