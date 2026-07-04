use dioxus::prelude::*;

/// The tier caption text, e.g. "Level 2 of 3".
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideTierLabelProps {
    #[props(into)]
    pub text: String,
}
