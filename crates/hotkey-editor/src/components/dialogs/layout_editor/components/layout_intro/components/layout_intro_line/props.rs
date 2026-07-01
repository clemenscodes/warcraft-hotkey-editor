use dioxus::prelude::*;

/// One line of the layout editor's intro copy.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutIntroLineProps {
    pub line: String,
}
