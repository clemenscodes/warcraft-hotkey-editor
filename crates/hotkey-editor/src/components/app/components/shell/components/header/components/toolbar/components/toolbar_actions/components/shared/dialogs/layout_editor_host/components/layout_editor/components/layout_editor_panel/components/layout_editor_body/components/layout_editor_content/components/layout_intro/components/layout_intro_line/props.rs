use super::view::LayoutIntroLineView;
use dioxus::prelude::*;

/// One line of the layout editor's intro copy.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutIntroLineProps {
    pub line: String,
}

impl From<&LayoutIntroLineView> for LayoutIntroLineProps {
    fn from(view: &LayoutIntroLineView) -> Self {
        let LayoutIntroLineView { line } = view.clone();
        Self { line }
    }
}

impl ddd::Props for LayoutIntroLineProps {
    type View = LayoutIntroLineView;
}
