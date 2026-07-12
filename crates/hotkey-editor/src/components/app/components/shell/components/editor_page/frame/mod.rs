use super::components::editor_tabs_bar::EditorTabsBarView;
use super::components::editor_workspace::EditorWorkspaceView;
use browser_kit::frame::Frame;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;

/// The editor page's frame: the mode/race tab bar header region above the workspace body
/// region. The editor page builds this and hands it to the headless `Page`, which places the
/// regions inside the styled page container. The page owns no footer — the shell owns the app
/// footer — so that region defaults to `Empty`.
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
