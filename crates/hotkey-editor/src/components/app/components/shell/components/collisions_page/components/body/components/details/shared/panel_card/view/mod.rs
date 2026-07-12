use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The published `View` contract for the shared conflict-panel card: the caller's body region,
/// placed inside the shared tinted, bordered conflict surface. Generic over the `Body` region
/// (a `Render`), so each panel's content supplies itself while sharing the one surface.
#[derive(Clone, PartialEq)]
pub struct PanelCardView<Body: Render<Output = Element>> {
    pub body: Body,
}

impl<Body: Render<Output = Element>> ddd::View for PanelCardView<Body> {}
