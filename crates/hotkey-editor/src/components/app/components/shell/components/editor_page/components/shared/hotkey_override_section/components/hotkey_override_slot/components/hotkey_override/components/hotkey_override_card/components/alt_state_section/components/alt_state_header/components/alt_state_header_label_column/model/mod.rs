use super::view::AltStateHeaderLabelColumnView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AltStateHeaderLabelColumnModel {
    pub text: Option<String>,
}

impl From<&AltStateHeaderLabelColumnView> for AltStateHeaderLabelColumnModel {
    fn from(view: &AltStateHeaderLabelColumnView) -> Self {
        let AltStateHeaderLabelColumnView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for AltStateHeaderLabelColumnModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for AltStateHeaderLabelColumnModel {
    type View = AltStateHeaderLabelColumnView;
}
