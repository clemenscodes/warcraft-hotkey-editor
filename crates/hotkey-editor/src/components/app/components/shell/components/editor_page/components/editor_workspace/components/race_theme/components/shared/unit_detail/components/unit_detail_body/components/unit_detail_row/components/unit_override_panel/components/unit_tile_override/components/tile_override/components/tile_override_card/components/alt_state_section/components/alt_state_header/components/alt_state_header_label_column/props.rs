use super::view::AltStateHeaderLabelColumnView;
use dioxus::prelude::*;

/// The label column of the off-state header row.
#[derive(Props, Clone, PartialEq)]
pub struct AltStateHeaderLabelColumnProps {
    pub text: Option<String>,
}

impl From<&AltStateHeaderLabelColumnView> for AltStateHeaderLabelColumnProps {
    fn from(view: &AltStateHeaderLabelColumnView) -> Self {
        let AltStateHeaderLabelColumnView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for AltStateHeaderLabelColumnProps {
    type View = AltStateHeaderLabelColumnView;
}
