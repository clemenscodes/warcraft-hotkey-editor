use dioxus::prelude::*;

/// A dialog title bar's inputs: the title to show and the handler fired when the
/// close control is clicked. Each dialog builds this itself — the close handler
/// writes that dialog's own open signal back to `false`.
#[derive(Props, Clone, PartialEq)]
pub struct DialogHeaderProps {
    #[props(into)]
    pub title: String,
    pub on_close: EventHandler<()>,
}
