use super::view::ToolbarButtonIconView;
use dioxus::prelude::*;

/// The toolbar button's glyph, an inline SVG string injected as inner HTML.
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarButtonIconProps {
    pub icon: &'static str,
}

impl From<&ToolbarButtonIconView> for ToolbarButtonIconProps {
    fn from(view: &ToolbarButtonIconView) -> Self {
        let ToolbarButtonIconView { icon } = view.clone();
        Self { icon }
    }
}

impl ddd::Props for ToolbarButtonIconProps {
    type View = ToolbarButtonIconView;
}
