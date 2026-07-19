use super::model::InfoPopoverModel;
use dioxus::prelude::*;

pub(super) struct InfoPopoverPresentation {
    pub(super) text: &'static str,
    pub(super) is_open: bool,
    pub(super) toggle: EventHandler<MouseEvent>,
    pub(super) dismiss: EventHandler<MouseEvent>,
}

pub(super) fn use_info_popover(props: &InfoPopoverModel) -> InfoPopoverPresentation {
    let text = props.text;
    let mut open = use_signal::<bool>(|| false);
    let is_open = *open.read();
    let toggle = EventHandler::new(move |event: MouseEvent| {
        event.stop_propagation();
        let next = !*open.peek();
        open.set(next);
    });
    let dismiss = EventHandler::new(move |event: MouseEvent| {
        event.stop_propagation();
        open.set(false);
    });
    InfoPopoverPresentation {
        text,
        is_open,
        toggle,
        dismiss,
    }
}

impl ddd::Presentation for InfoPopoverPresentation {
    type Model = InfoPopoverModel;
}
