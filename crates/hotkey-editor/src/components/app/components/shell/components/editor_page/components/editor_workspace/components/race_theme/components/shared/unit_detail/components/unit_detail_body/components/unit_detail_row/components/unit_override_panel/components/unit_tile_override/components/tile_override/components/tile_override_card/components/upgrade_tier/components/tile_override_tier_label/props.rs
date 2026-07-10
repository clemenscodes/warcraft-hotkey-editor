use super::view::TileOverrideTierLabelView;
use dioxus::prelude::*;

/// The tier caption text, e.g. "Level 2 of 3".
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideTierLabelProps {
    #[props(into)]
    pub text: String,
}

impl From<&TileOverrideTierLabelView> for TileOverrideTierLabelProps {
    fn from(view: &TileOverrideTierLabelView) -> Self {
        let TileOverrideTierLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for TileOverrideTierLabelProps {
    type View = TileOverrideTierLabelView;
}
