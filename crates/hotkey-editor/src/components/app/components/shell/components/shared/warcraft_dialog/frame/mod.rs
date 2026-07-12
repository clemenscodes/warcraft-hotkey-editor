use super::components::warcraft_dialog_header::WarcraftDialogHeaderView;
use browser_kit::frame::{Frame, Render};
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;

/// The dialog's frame: the fixed title/close header region plus the caller's body region.
/// WarcraftDialog builds this and hands it to the headless `Dialog`, which places the
/// regions inside the styled content box. There is no footer region — a dialog with footer
/// actions defines that when it migrates.
#[derive(Clone, PartialEq, Default)]
pub struct WarcraftDialogFrame<Body: Render<Output = Element>> {
    pub(super) header: WarcraftDialogHeaderView,
    pub(super) body: Body,
}

impl<Body: Render<Output = Element>> Frame for WarcraftDialogFrame<Body> {
    type Output = Element;
    type Header = WarcraftDialogHeaderView;
    type Body = Body;
    type Footer = Empty;

    fn body(&self) -> Self::Body {
        self.body.clone()
    }

    fn header(&self) -> Option<Self::Header> {
        let header = self.header.clone();
        Some(header)
    }
}
