use dioxus::prelude::*;

use super::super::super::DialogPanelProps;

/// The header's inputs: the title to show and the handler that closes the
/// dialog. The close handler is wired here to write the shared open signal back
/// to `false`.
#[derive(Props, Clone, PartialEq)]
pub struct DialogHeaderProps {
    pub title: String,
    pub on_close: EventHandler<()>,
}

impl From<&DialogPanelProps> for DialogHeaderProps {
    fn from(props: &DialogPanelProps) -> Self {
        let mut open_signal = props.open;
        let title = props.title.clone();
        let on_close = EventHandler::new(move |()| open_signal.set(false));
        Self { title, on_close }
    }
}
