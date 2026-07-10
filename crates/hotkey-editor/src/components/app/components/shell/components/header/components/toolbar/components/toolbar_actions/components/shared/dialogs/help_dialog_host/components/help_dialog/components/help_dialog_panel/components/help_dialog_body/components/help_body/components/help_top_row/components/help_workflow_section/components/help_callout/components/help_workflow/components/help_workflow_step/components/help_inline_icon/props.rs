use super::view::HelpInlineIconView;
use dioxus::prelude::*;

/// The inline icon's only input: the inline SVG markup to draw.
#[derive(Props, Clone, PartialEq)]
pub struct HelpInlineIconProps {
    pub icon: &'static str,
}

impl From<&HelpInlineIconView> for HelpInlineIconProps {
    fn from(view: &HelpInlineIconView) -> Self {
        let HelpInlineIconView { icon } = view.clone();
        Self { icon }
    }
}

impl ddd::Props for HelpInlineIconProps {
    type View = HelpInlineIconView;
}
