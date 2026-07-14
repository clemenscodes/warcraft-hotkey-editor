use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct WarcraftDialogView<Body: Render<Output = Element>, Footer: Render<Output = Element>> {
    pub body: Body,
    pub footer: Footer,
    pub title: String,
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl<Body: Render<Output = Element>, Footer: Render<Output = Element>> ddd::View
    for WarcraftDialogView<Body, Footer>
{}
