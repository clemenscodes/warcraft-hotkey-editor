use browser_kit::frame::{Frame, Render};
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;

/// The conflict-panel card's frame: just the caller's body region. `PanelCard` builds this and
/// hands it to the headless `Card`, which places the body inside the styled conflict surface.
/// The card has no header or footer, so those regions default to `None`.
#[derive(Clone, PartialEq, Default)]
pub struct PanelCardFrame<Body: Render<Output = Element>> {
    pub(super) body: Body,
}

impl<Body: Render<Output = Element>> Frame for PanelCardFrame<Body> {
    type Output = Element;
    type Header = Empty;
    type Body = Body;
    type Footer = Empty;

    fn body(&self) -> Self::Body {
        self.body.clone()
    }
}
