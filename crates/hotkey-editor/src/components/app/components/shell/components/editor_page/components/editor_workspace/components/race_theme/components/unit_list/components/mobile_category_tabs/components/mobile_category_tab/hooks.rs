use super::logic::{MobileCategoryTabInputs, MobileCategoryTabModel};
use crate::services::editor_state::context::use_editor_state;
use dioxus::prelude::*;
use warcraft_api::UnitKind;

/// The tab's dispatched view: whether its kind is the active category (which look to
/// render) and the shaped model both looks are built from. Reads the active-category
/// signal from editor context, so the tab is fed only its kind.
pub(super) struct MobileCategoryTabView {
    pub(super) is_active: bool,
    pub(super) model: MobileCategoryTabModel,
}

pub(super) fn use_mobile_category_tab(kind: UnitKind) -> MobileCategoryTabView {
    let active_category = use_editor_state().active_category();
    let is_active = *active_category.read() == kind;
    let inputs = MobileCategoryTabInputs {
        kind,
        active_category,
    };
    let model = MobileCategoryTabModel::from(inputs);
    MobileCategoryTabView { is_active, model }
}
