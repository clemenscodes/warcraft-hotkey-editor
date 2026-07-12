use super::view::AltStateLabelView;
use dioxus::prelude::*;

/// The alt-state caption (e.g. "Upgraded form" or an off-state name). Optional: an
/// off-state without a distinct name renders no label.
#[derive(Props, Clone, PartialEq)]
pub struct AltStateLabelModel {
    pub text: Option<String>,
}

impl From<&AltStateLabelView> for AltStateLabelModel {
    fn from(view: &AltStateLabelView) -> Self {
        let AltStateLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for AltStateLabelModel {
    type View = AltStateLabelView;
}
