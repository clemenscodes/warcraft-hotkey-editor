use super::view::WarcraftPageView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WarcraftPageModel<Header: Render<Output = Element>, Body: Render<Output = Element>> {
    pub header: Header,
    pub body: Body,
}

impl<Header: Render<Output = Element>, Body: Render<Output = Element>>
    From<&WarcraftPageView<Header, Body>> for WarcraftPageModel<Header, Body>
{
    fn from(view: &WarcraftPageView<Header, Body>) -> Self {
        let WarcraftPageView { header, body } = view.clone();
        Self { header, body }
    }
}

impl<Header: Render<Output = Element>, Body: Render<Output = Element>> ddd::Model
    for WarcraftPageModel<Header, Body>
{
    type View = WarcraftPageView<Header, Body>;
}
