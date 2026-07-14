use super::view::AltStateHeaderLabelColumnView;
use dioxus::prelude::*;

/// The label column of the off-state header row.
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

impl ddd::Model for AltStateHeaderLabelColumnModel {
    type View = AltStateHeaderLabelColumnView;
}
