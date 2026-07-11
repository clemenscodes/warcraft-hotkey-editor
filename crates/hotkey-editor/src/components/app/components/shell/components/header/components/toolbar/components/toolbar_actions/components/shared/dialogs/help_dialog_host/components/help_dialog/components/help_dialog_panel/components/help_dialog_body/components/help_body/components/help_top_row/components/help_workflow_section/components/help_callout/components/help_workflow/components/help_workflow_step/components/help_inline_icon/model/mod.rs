use super::view::HelpInlineIconView;
use dioxus::prelude::*;

/// The inline icon's only input: the inline SVG markup to draw.
#[derive(Props, Clone, PartialEq)]
pub struct HelpInlineIconModel {
    pub icon: &'static str,
}

impl From<&HelpInlineIconView> for HelpInlineIconModel {
    fn from(view: &HelpInlineIconView) -> Self {
        let HelpInlineIconView { icon } = view.clone();
        Self { icon }
    }
}

impl ddd::Model for HelpInlineIconModel {
    type View = HelpInlineIconView;
}
