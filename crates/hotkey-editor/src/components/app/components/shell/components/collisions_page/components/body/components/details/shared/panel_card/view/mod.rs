use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PanelCardView<Body: Render<Output = Element>> {
    pub body: Body,
}

impl<Body: Render<Output = Element>> ddd::View for PanelCardView<Body> {}
