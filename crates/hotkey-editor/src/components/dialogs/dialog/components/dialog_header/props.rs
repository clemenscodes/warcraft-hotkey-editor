use dioxus::prelude::*;

use crate::components::dialogs::dialog::DialogProps;

/// A dialog title bar's inputs: the title to show and the handler fired when the
/// close control is clicked. Built from the dialog props (the close handler
/// writes the shared open signal back to `false`); hand-rolled callers that still
/// render this directly pass `title` and `on_close` themselves.
#[derive(Props, Clone, PartialEq)]
pub struct DialogHeaderProps {
    pub title: String,
    pub on_close: EventHandler<()>,
}

impl From<&DialogProps> for DialogHeaderProps {
    fn from(props: &DialogProps) -> Self {
        let mut open_signal = props.open;
        let title = props.title.clone();
        let on_close = EventHandler::new(move |()| open_signal.set(false));
        Self { title, on_close }
    }
}
