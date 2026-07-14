use super::view::HelpDismissView;
use dioxus::prelude::*;

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
