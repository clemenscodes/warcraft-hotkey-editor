use super::view::PreviewDialogPanelView;
use dioxus::prelude::*;

/// The preview dialog's bordered box: the header title and its close handler above the
/// scrolling body's serialized text, wrapped in the library `DialogContent` (which
/// carries no project class — this panel's own classed `div` is the box).
#[derive(Props, Clone, PartialEq)]
pub struct PreviewDialogPanelProps {
    #[props(into)]
    pub title: String,
    pub on_close: EventHandler<()>,
    pub text: ReadSignal<String>,
}

impl From<&PreviewDialogPanelView> for PreviewDialogPanelProps {
    fn from(view: &PreviewDialogPanelView) -> Self {
        let PreviewDialogPanelView {
            title,
            on_close,
            text,
        } = view.clone();
        Self {
            title,
            on_close,
            text,
        }
    }
}

impl ddd::Props for PreviewDialogPanelProps {
    type View = PreviewDialogPanelView;
}
