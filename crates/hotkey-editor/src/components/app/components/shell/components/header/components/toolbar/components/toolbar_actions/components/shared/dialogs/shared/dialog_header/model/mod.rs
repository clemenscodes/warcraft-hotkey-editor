use super::view::DialogHeaderView;
use dioxus::prelude::*;

/// A dialog title bar's inputs: the title to show and the handler fired when the
/// close control is clicked. Each dialog builds this itself — the close handler
/// writes that dialog's own open signal back to `false`.
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
