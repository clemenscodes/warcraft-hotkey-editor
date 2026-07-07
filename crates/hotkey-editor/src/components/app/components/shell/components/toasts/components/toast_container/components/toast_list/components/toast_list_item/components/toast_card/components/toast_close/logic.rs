use super::props::ToastCloseProps;
use dioxus::prelude::*;

/// The close button's presentation: the click handler that dismisses this toast.
pub struct ToastClosePresentation {
    pub(super) onclick: EventHandler<MouseEvent>,
}

impl From<&ToastCloseProps> for ToastClosePresentation {
    fn from(props: &ToastCloseProps) -> Self {
        let id = props.id;
        let on_remove = props.on_remove;
        let onclick = EventHandler::new(move |_event: MouseEvent| {
            on_remove.call(id);
        });
        Self { onclick }
    }
}
