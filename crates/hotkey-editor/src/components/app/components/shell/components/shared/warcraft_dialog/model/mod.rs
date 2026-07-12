use super::view::WarcraftDialogView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The reusable Warcraft dialog's own props: the caller-supplied body region, the
/// caller-supplied footer region (the pinned bar below the body — `Empty` when the dialog
/// has none), the dialog title, and its open state plus change handler. Generic over the
/// body `Body` and footer `Footer` regions — the fixed title/close header is the dialog's
/// own chrome, not a field. No `Signal<T>` crosses here — `open` rides as a plain `bool`.
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
