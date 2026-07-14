use super::WarcraftDialogHeader;
use super::model::WarcraftDialogHeaderModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

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
            WarcraftDialogHeader {
                title,
                on_close,
            }
        }
    }
}
