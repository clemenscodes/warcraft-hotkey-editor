use dioxus::prelude::*;
use warcraft_api::UnitKind;

pub(super) struct MobileCategoryTabInputs {
    pub(super) kind: UnitKind,
    pub(super) active_category: Signal<UnitKind>,
}

pub(super) struct MobileCategoryTabPresentation {
    label: &'static str,
    onclick: EventHandler<MouseEvent>,
}

impl From<MobileCategoryTabInputs> for MobileCategoryTabPresentation {
    fn from(inputs: MobileCategoryTabInputs) -> Self {
        let kind = inputs.kind;
        let label = kind.category_label();
        let mut active_category = inputs.active_category;
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            active_category.set(kind);
        });
        Self { label, onclick }
    }
}

impl MobileCategoryTabPresentation {
    pub(super) fn label(&self) -> &'static str {
        self.label
    }

    pub(super) fn onclick(&self) -> EventHandler<MouseEvent> {
        self.onclick
    }
}
use crate::services::editor_state::context::use_editor_state;

pub(super) struct MobileCategoryTabDispatch {
    pub(super) is_active: bool,
    pub(super) model: MobileCategoryTabPresentation,
}

pub(super) fn use_mobile_category_tab(kind: UnitKind) -> MobileCategoryTabDispatch {
    let active_category = use_editor_state().active_category();
    let is_active = *active_category.read() == kind;
    let inputs = MobileCategoryTabInputs {
        kind,
        active_category,
    };
    let model = MobileCategoryTabPresentation::from(inputs);
    MobileCategoryTabDispatch { is_active, model }
}
