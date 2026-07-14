use super::view::HelpFooterView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HelpFooterModel {
    pub on_dismiss: EventHandler<MouseEvent>,
}

impl From<&HelpFooterView> for HelpFooterModel {
    fn from(view: &HelpFooterView) -> Self {
        let dismiss = view.on_dismiss;
        let on_dismiss = EventHandler::new(move |event: MouseEvent| dismiss.call(event));
        Self { on_dismiss }
    }
}

impl ddd::Model for HelpFooterModel {
    type View = HelpFooterView;
}
