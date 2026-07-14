use super::view::WarcraftDialogView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WarcraftDialogModel<Body: Render<Output = Element>, Footer: Render<Output = Element>> {
    pub body: Body,
    #[props(default)]
    pub footer: Footer,
    #[props(into)]
    pub title: String,
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl<Body: Render<Output = Element>, Footer: Render<Output = Element>>
    From<&WarcraftDialogView<Body, Footer>> for WarcraftDialogModel<Body, Footer>
{
    fn from(view: &WarcraftDialogView<Body, Footer>) -> Self {
        let WarcraftDialogView {
            body,
            footer,
            title,
            open,
            on_open_change,
        } = view.clone();
        Self {
            body,
            footer,
            title,
            open,
            on_open_change,
        }
    }
}

impl<Body: Render<Output = Element>, Footer: Render<Output = Element>> ddd::Model
    for WarcraftDialogModel<Body, Footer>
{
    type View = WarcraftDialogView<Body, Footer>;
}
