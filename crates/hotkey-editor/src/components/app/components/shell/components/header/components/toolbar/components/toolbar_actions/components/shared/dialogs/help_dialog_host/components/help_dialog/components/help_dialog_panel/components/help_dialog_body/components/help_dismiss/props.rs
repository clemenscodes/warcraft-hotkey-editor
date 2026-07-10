use super::view::HelpDismissView;
use dioxus::prelude::*;

/// The dismiss button's only input: the click handler that marks the onboarding seen
/// and closes the dialog. The dialog shell builds it and hands it down.
#[derive(Props, Clone, PartialEq)]
pub struct HelpDismissProps {
    pub on_dismiss: EventHandler<MouseEvent>,
}

impl From<&HelpDismissView> for HelpDismissProps {
    fn from(view: &HelpDismissView) -> Self {
        let HelpDismissView { on_dismiss } = view.clone();
        Self { on_dismiss }
    }
}

impl ddd::Props for HelpDismissProps {
    type View = HelpDismissView;
}
