use super::components::layout_editor_content::LayoutEditorContentProps;
use dioxus::prelude::*;

/// The layout editor's scroll region input: the centered content column it holds.
#[derive(Props, Clone, PartialEq)]
pub struct LayoutEditorBodyProps {
    pub content: LayoutEditorContentProps,
}

impl From<&LayoutEditorBodyProps> for LayoutEditorContentProps {
    fn from(props: &LayoutEditorBodyProps) -> Self {
        props.content.clone()
    }
}
