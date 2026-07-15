use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct WarcraftPageView<Header: Render<Output = Element>, Body: Render<Output = Element>> {
    pub header: Header,
    pub body: Body,
}

impl<Header: Render<Output = Element>, Body: Render<Output = Element>> ddd::View
    for WarcraftPageView<Header, Body>
{
}
