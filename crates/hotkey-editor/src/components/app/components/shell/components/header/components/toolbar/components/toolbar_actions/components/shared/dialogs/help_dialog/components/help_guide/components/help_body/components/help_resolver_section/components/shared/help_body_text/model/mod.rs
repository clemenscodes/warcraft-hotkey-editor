use super::view::HelpBodyTextView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HelpBodyTextModel {
    #[props(into)]
    pub text: String,
}

impl From<&HelpBodyTextView> for HelpBodyTextModel {
    fn from(view: &HelpBodyTextView) -> Self {
        let HelpBodyTextView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for HelpBodyTextModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for HelpBodyTextModel {
    type View = HelpBodyTextView;
}
