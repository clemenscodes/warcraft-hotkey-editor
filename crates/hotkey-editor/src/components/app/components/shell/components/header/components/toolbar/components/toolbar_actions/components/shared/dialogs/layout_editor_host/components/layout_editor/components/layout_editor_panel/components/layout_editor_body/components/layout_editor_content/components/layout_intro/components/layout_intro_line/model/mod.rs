use super::view::LayoutIntroLineView;
use dioxus::prelude::*;

/// One line of the layout editor's intro copy.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutIntroLineModel {
    pub line: String,
}

impl From<&LayoutIntroLineView> for LayoutIntroLineModel {
    fn from(view: &LayoutIntroLineView) -> Self {
        let LayoutIntroLineView { line } = view.clone();
        Self { line }
    }
}

impl ddd::Model for LayoutIntroLineModel {
    type View = LayoutIntroLineView;
}
