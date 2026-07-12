use super::view::AltStateLineView;
use dioxus::prelude::*;

/// One description line of the alt-state block.
#[derive(Props, Clone, PartialEq)]
pub struct AltStateLineModel {
    #[props(into)]
    pub text: String,
}

impl From<&AltStateLineView> for AltStateLineModel {
    fn from(view: &AltStateLineView) -> Self {
        let AltStateLineView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for AltStateLineModel {
    type View = AltStateLineView;
}
