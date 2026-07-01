use dioxus::prelude::*;

use super::super::super::DialogHeaderProps;

/// The close control's only input: the click handler, already adapted from the
/// header's `on_close` so the body just places it.
#[derive(Props, Clone, PartialEq)]
pub struct DialogCloseProps {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&DialogHeaderProps> for DialogCloseProps {
    fn from(props: &DialogHeaderProps) -> Self {
        let on_close = props.on_close;
        let onclick = EventHandler::new(move |_event: MouseEvent| on_close.call(()));
        Self { onclick }
    }
}
