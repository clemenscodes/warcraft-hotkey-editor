use super::view::DialogHeaderView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DialogHeaderModel {
    #[props(into)]
    pub title: String,
    pub on_close: EventHandler<()>,
}

impl From<&DialogHeaderView> for DialogHeaderModel {
    fn from(view: &DialogHeaderView) -> Self {
        let DialogHeaderView { title, on_close } = view.clone();
        Self { title, on_close }
    }
}

impl ddd::Model for DialogHeaderModel {
    type View = DialogHeaderView;
}
