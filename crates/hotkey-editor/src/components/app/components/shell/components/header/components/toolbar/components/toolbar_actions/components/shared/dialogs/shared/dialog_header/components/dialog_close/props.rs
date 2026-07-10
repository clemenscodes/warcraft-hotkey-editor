use super::view::DialogCloseView;
use dioxus::prelude::*;

/// The close control's only input: the click handler, already adapted from the
/// header's `on_close` so the body just places it.
#[derive(Props, Clone, PartialEq)]
pub struct DialogCloseProps {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&DialogCloseView> for DialogCloseProps {
    fn from(view: &DialogCloseView) -> Self {
        let DialogCloseView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Props for DialogCloseProps {
    type View = DialogCloseView;
}
