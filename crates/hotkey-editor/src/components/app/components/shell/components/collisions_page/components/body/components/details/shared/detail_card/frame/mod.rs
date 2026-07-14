use browser_kit::frame::{Frame, Render};
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;

#[derive(Clone, PartialEq, Default)]
pub struct DetailCardFrame<Body: Render<Output = Element>> {
    pub(super) body: Body,
}

impl<Body: Render<Output = Element>> Frame for DetailCardFrame<Body> {
    type Output = Element;
    type Header = Empty;
    type Body = Body;
    type Footer = Empty;

    fn body(&self) -> Self::Body {
        self.body.clone()
    }
}
