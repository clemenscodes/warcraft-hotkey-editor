use dioxus::prelude::*;

/// The resolved portrait image source and its alt text.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardPortraitProps {
    #[props(into)]
    pub source: String,
    #[props(into)]
    pub display_name: String,
}
