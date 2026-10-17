use super::view::RegenQualifierView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RegenQualifierModel {
    #[props(default)]
    pub text: Option<&'static str>,
}

impl From<&RegenQualifierView> for RegenQualifierModel {
    fn from(view: &RegenQualifierView) -> Self {
        let RegenQualifierView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for RegenQualifierModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for RegenQualifierModel {
    type View = RegenQualifierView;
}
