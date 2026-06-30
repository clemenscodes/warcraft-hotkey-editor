use dioxus::prelude::*;

/// The inline icon's only input: the inline SVG markup to draw.
#[derive(Props, Clone, PartialEq)]
pub struct HelpInlineIconProps {
    pub icon: &'static str,
}
