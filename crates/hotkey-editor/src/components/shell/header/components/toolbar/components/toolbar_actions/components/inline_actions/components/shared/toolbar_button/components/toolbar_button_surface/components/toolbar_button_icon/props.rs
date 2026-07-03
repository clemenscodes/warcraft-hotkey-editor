use dioxus::prelude::*;

/// The toolbar button's glyph, an inline SVG string injected as inner HTML.
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarButtonIconProps {
    pub icon: &'static str,
}
