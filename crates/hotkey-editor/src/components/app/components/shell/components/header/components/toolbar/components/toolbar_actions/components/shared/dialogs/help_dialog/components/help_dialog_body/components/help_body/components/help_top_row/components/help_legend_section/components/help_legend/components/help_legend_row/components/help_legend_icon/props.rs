use dioxus::prelude::*;

/// The legend icon's only input: the inline SVG markup to draw.
#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendIconProps {
    pub icon: &'static str,
}
