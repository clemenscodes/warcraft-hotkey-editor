use super::view::HelpLegendIconView;
use dioxus::prelude::*;

/// The legend icon's only input: the inline SVG markup to draw.
#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendIconProps {
    pub icon: &'static str,
}

impl From<&HelpLegendIconView> for HelpLegendIconProps {
    fn from(view: &HelpLegendIconView) -> Self {
        let HelpLegendIconView { icon } = view.clone();
        Self { icon }
    }
}

impl ddd::Props for HelpLegendIconProps {
    type View = HelpLegendIconView;
}
