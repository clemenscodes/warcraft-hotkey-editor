use super::view::ToolbarButtonIconView;
use dioxus::prelude::*;

/// The toolbar button's glyph, an inline SVG string injected as inner HTML.
#[derive(Props, Clone, PartialEq)]
pub struct ToolbarButtonIconModel {
    pub icon: &'static str,
}

impl From<&ToolbarButtonIconView> for ToolbarButtonIconModel {
    fn from(view: &ToolbarButtonIconView) -> Self {
        let ToolbarButtonIconView { icon } = view.clone();
        Self { icon }
    }
}

impl ddd::Model for ToolbarButtonIconModel {
    type View = ToolbarButtonIconView;
}
