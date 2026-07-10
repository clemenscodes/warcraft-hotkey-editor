use super::view::ClearLabelView;
use dioxus::prelude::*;

/// The "all clear" line under the glyph; its exact wording is page-specific ("All
/// clear." on the collisions page, "Nothing to resolve." on the resolve page).
#[derive(Props, Clone, PartialEq)]
pub struct ClearLabelProps {
    #[props(into)]
    pub text: String,
}

impl From<&ClearLabelView> for ClearLabelProps {
    fn from(view: &ClearLabelView) -> Self {
        let ClearLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for ClearLabelProps {
    type View = ClearLabelView;
}
