use super::view::DialogCloseHostView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DialogCloseHostModel {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&DialogCloseHostView> for DialogCloseHostModel {
    fn from(view: &DialogCloseHostView) -> Self {
        let DialogCloseHostView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Model for DialogCloseHostModel {
    type View = DialogCloseHostView;
}
