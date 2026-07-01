use dioxus::prelude::*;

/// The unit portrait: its source (absent for units without an image) and alt text.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPortraitProps {
    pub src: Option<String>,
    pub alt: &'static str,
}
