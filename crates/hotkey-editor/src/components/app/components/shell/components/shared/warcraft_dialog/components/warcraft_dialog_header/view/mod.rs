use super::WarcraftDialogHeader;
use super::model::WarcraftDialogHeaderModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The published `View` for the dialog header — the title to show and the close callback
/// fired by the ✕ control. It is also the frame's header region: it `impl Render`, so a
/// dialog places the published `View` directly, with no ad-hoc region type.
#[derive(Clone, PartialEq, Default)]
pub struct WarcraftDialogHeaderView {
    pub title: String,
    pub on_close: Callback<()>,
}

impl ddd::View for WarcraftDialogHeaderView {}

impl Render for WarcraftDialogHeaderView {
    type Model = WarcraftDialogHeaderModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let title = self.title.clone();
        let on_close = self.on_close;
        rsx! {
            WarcraftDialogHeader { title, on_close }
        }
    }
}
