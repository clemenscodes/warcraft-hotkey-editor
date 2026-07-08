use dioxus::prelude::*;

/// The "all clear" line under the glyph; its exact wording is page-specific ("All
/// clear." on the collisions page, "Nothing to resolve." on the resolve page).
#[derive(Props, Clone, PartialEq)]
pub struct ClearLabelProps {
    #[props(into)]
    pub text: String,
}
