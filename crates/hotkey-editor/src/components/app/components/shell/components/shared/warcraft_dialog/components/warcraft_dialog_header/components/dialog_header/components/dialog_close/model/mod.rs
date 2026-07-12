use super::view::DialogCloseView;
use dioxus::prelude::*;

/// The close control's only input: the click handler, already adapted from the
/// header's `on_close` so the body just places it.
#[derive(Props, Clone, PartialEq)]
pub struct DialogCloseModel {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&DialogCloseView> for DialogCloseModel {
    fn from(view: &DialogCloseView) -> Self {
        let DialogCloseView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Model for DialogCloseModel {
    type View = DialogCloseView;
}
