use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DetailCardView<Body: Render<Output = Element>> {
    pub body: Body,
}

impl<Body: Render<Output = Element>> ddd::View for DetailCardView<Body> {}
