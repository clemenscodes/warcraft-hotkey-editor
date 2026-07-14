use super::model::HelpDialogModel;
use crate::persistence::onboarding_persistence;
use dioxus::prelude::*;

pub(super) struct HelpDialogPresentation {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) on_dismiss: Callback<MouseEvent>,
}

impl From<&HelpDialogModel> for HelpDialogPresentation {
    fn from(model: &HelpDialogModel) -> Self {
        let open = model.open;
        let on_open_change = model.on_open_change;
        let on_dismiss = Callback::new(move |_event: MouseEvent| {
            onboarding_persistence::mark_seen();
            on_open_change.call(false);
        });
        Self {
            open,
            on_open_change,
            on_dismiss,
        }
    }
}

impl ddd::Presentation for HelpDialogPresentation {
    type Model = HelpDialogModel;
}
