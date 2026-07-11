use super::view::TileOverrideTierLabelView;
use dioxus::prelude::*;

/// The tier caption text, e.g. "Level 2 of 3".
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideTierLabelModel {
    #[props(into)]
    pub text: String,
}

impl From<&TileOverrideTierLabelView> for TileOverrideTierLabelModel {
    fn from(view: &TileOverrideTierLabelView) -> Self {
        let TileOverrideTierLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for TileOverrideTierLabelModel {
    type View = TileOverrideTierLabelView;
}
