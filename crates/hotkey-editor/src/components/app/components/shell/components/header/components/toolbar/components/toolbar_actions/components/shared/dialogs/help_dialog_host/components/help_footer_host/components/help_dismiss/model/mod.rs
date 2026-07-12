use super::view::HelpDismissView;
use dioxus::prelude::*;

/// The dismiss button's only input: the click handler that marks the onboarding seen
/// and closes the dialog. The dialog shell builds it and hands it down.
#[derive(Props, Clone, PartialEq)]
pub struct HelpDismissModel {
    pub on_dismiss: EventHandler<MouseEvent>,
}

impl From<&HelpDismissView> for HelpDismissModel {
    fn from(view: &HelpDismissView) -> Self {
        let HelpDismissView { on_dismiss } = view.clone();
        Self { on_dismiss }
    }
}

impl ddd::Model for HelpDismissModel {
    type View = HelpDismissView;
}
