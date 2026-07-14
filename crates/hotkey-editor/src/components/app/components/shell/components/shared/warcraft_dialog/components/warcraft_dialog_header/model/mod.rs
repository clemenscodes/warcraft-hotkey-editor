use super::view::WarcraftDialogHeaderView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WarcraftDialogHeaderModel {
    #[props(into)]
    pub title: String,
    pub on_close: Callback<()>,
}

impl From<&WarcraftDialogHeaderView> for WarcraftDialogHeaderModel {
    fn from(view: &WarcraftDialogHeaderView) -> Self {
        let WarcraftDialogHeaderView { title, on_close } = view.clone();
        Self { title, on_close }
    }
}

impl ddd::Model for WarcraftDialogHeaderModel {
    type View = WarcraftDialogHeaderView;
}
