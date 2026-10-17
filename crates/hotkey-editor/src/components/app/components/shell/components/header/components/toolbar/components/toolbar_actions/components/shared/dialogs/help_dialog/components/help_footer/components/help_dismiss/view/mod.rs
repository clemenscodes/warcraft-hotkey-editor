use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct HelpDismissView {
    pub on_dismiss: EventHandler<MouseEvent>,
}

impl ddd::Layered for HelpDismissView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HelpDismissView {}
