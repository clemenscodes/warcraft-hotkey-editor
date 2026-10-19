use super::model::InfoPopoverModel;
use crate::components::app::components::shell::components::shared::exit_presence::use_exit_presence;
use dioxus::prelude::*;

pub(super) struct InfoPopoverPresentation {
    pub(super) text: &'static str,
    pub(super) is_open: bool,
    pub(super) is_mounted: bool,
    pub(super) is_leaving: bool,
    pub(super) toggle: EventHandler<MouseEvent>,
    pub(super) dismiss: EventHandler<MouseEvent>,
}

pub(super) fn use_info_popover(props: &InfoPopoverModel) -> InfoPopoverPresentation {
    let text = props.text;
    let mut bubble = use_exit_presence();
    let presence = bubble.presence();
    let is_open = presence.is_shown();
    let is_mounted = presence.is_mounted();
    let is_leaving = presence.is_leaving();
    let toggle = EventHandler::new(move |event: MouseEvent| {
        event.stop_propagation();
        bubble.toggle();
    });
    let dismiss = EventHandler::new(move |event: MouseEvent| {
        event.stop_propagation();
        bubble.close();
    });
    InfoPopoverPresentation {
        text,
        is_open,
        is_mounted,
        is_leaving,
        toggle,
        dismiss,
    }
}

impl ddd::Layered for InfoPopoverPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for InfoPopoverPresentation {
    type Model = InfoPopoverModel;
}
