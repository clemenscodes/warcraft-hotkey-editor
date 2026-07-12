use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The published `View` contract for the reusable Warcraft dialog: the caller-supplied
/// body region, the caller-supplied footer region (the pinned bar below the body — use
/// `Empty` for a dialog without one), the dialog title, the open value, and the change
/// handler the headless `Dialog` fires on escape, outside click, or a programmatic close.
/// The fixed title/close header is the dialog's own chrome, so it is not part of the
/// contract.
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
{
}
