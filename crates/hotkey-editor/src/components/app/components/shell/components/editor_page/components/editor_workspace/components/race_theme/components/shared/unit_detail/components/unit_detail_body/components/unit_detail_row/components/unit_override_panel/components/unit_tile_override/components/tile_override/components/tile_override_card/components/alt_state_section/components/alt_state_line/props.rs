use super::view::AltStateLineView;
use dioxus::prelude::*;

/// One description line of the alt-state block.
#[derive(Props, Clone, PartialEq)]
pub struct AltStateLineProps {
    #[props(into)]
    pub text: String,
}

impl From<&AltStateLineView> for AltStateLineProps {
    fn from(view: &AltStateLineView) -> Self {
        let AltStateLineView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for AltStateLineProps {
    type View = AltStateLineView;
}
