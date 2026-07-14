use super::components::editor_tabs_bar::EditorTabsBarView;
use super::components::editor_workspace::EditorWorkspaceView;
use browser_kit::frame::Frame;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;

#[derive(Clone, PartialEq, Default)]
pub struct EditorPageFrame {
    pub(super) header: EditorTabsBarView,
    pub(super) body: EditorWorkspaceView,
}

impl Frame for EditorPageFrame {
    type Output = Element;
    type Header = EditorTabsBarView;
    type Body = EditorWorkspaceView;
    type Footer = Empty;

    fn body(&self) -> Self::Body {
        self.body.clone()
    }

    fn header(&self) -> Option<Self::Header> {
        let header = self.header.clone();
        Some(header)
    }
}
