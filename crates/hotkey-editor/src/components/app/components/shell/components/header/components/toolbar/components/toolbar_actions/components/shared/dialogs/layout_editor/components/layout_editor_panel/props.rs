use super::components::layout_editor_body::LayoutEditorBodyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The layout editor's bordered box: the header row above the scrolling body, wrapped
/// in the library `DialogContent` (which carries no project class — this panel's own
/// classed `div` is the box).
#[derive(Props, Clone, PartialEq)]
pub struct LayoutEditorPanelProps {
    pub header: DialogHeaderProps,
    pub body: LayoutEditorBodyProps,
}

impl From<&LayoutEditorPanelProps> for DialogHeaderProps {
    fn from(props: &LayoutEditorPanelProps) -> Self {
        props.header.clone()
    }
}

impl From<&LayoutEditorPanelProps> for LayoutEditorBodyProps {
    fn from(props: &LayoutEditorPanelProps) -> Self {
        props.body.clone()
    }
}
