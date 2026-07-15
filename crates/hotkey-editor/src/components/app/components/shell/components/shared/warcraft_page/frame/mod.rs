use browser_kit::frame::{Frame, Render};
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;

#[derive(Clone, PartialEq, Default)]
pub struct WarcraftPageFrame<Header: Render<Output = Element>, Body: Render<Output = Element>> {
    pub(super) header: Header,
    pub(super) body: Body,
}

impl<Header: Render<Output = Element>, Body: Render<Output = Element>> Frame
    for WarcraftPageFrame<Header, Body>
{
    type Output = Element;
    type Header = Header;
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
