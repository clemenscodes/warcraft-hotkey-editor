use super::view::WarcraftDialogView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The reusable Warcraft dialog's own props: the caller-supplied body region, the dialog
/// title, and its open state plus change handler. Generic over the body `Body` only — the
/// fixed title/close header is the dialog's own chrome, not a field. No `Signal<T>`
/// crosses here — `open` rides as a plain `bool`.
#[derive(Props, Clone, PartialEq)]
pub struct WarcraftDialogModel<Body: Render<Output = Element>> {
    pub body: Body,
    #[props(into)]
    pub title: String,
    pub open: bool,
    pub on_open_change: Callback<bool>,
}

impl<Body: Render<Output = Element>> From<&WarcraftDialogView<Body>> for WarcraftDialogModel<Body> {
    fn from(view: &WarcraftDialogView<Body>) -> Self {
        let WarcraftDialogView {
            body,
            title,
            open,
            on_open_change,
        } = view.clone();
        Self {
            body,
            title,
            open,
            on_open_change,
        }
    }
}

impl<Body: Render<Output = Element>> ddd::Model for WarcraftDialogModel<Body> {
    type View = WarcraftDialogView<Body>;
}
