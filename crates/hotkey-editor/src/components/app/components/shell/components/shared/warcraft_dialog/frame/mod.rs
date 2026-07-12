use super::components::warcraft_dialog_header::WarcraftDialogHeaderView;
use browser_kit::frame::{Frame, Render};
use dioxus::prelude::*;

/// The dialog's frame: the fixed title/close header region, the caller's body region, and
/// the caller's footer region (the pinned bar below the body — `Empty` when the dialog has
/// none). WarcraftDialog builds this and hands it to the headless `Dialog`, which places
/// the regions inside the styled content box.
#[derive(Clone, PartialEq, Default)]
pub struct WarcraftDialogFrame<Body: Render<Output = Element>, Footer: Render<Output = Element>> {
    pub(super) header: WarcraftDialogHeaderView,
    pub(super) body: Body,
    pub(super) footer: Footer,
}

impl<Body: Render<Output = Element>, Footer: Render<Output = Element>> Frame
    for WarcraftDialogFrame<Body, Footer>
{
    type Output = Element;
    type Header = WarcraftDialogHeaderView;
    type Body = Body;
    type Footer = Footer;

    fn body(&self) -> Self::Body {
        self.body.clone()
    }

    fn header(&self) -> Option<Self::Header> {
        let header = self.header.clone();
        Some(header)
    }

    fn footer(&self) -> Option<Self::Footer> {
        let footer = self.footer.clone();
        Some(footer)
    }
}
