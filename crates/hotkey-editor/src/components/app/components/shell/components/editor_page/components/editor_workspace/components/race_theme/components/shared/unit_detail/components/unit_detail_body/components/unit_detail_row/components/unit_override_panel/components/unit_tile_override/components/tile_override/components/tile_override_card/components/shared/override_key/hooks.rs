use crate::services::focus::context::use_focus_coordinator;
use dioxus::prelude::*;
use std::rc::Rc;

/// The override key's shared behaviour, wired once and worn by whichever look-variant
/// the dispatcher renders: the mount-time focus registration and the click handler each
/// variant attaches to its own button root. The primary key cell registers itself as
/// the override-key focus target while it is on screen, so a keyboard tile selection can
/// hand focus on to it by state.
#[derive(Clone, Copy)]
pub(super) struct OverrideKeyHandlers {
    on_mounted: EventHandler<Event<MountedData>>,
    on_click: EventHandler<Event<MouseData>>,
}

impl OverrideKeyHandlers {
    pub(super) fn on_mounted(&self) -> EventHandler<Event<MountedData>> {
        self.on_mounted
    }

    pub(super) fn on_click(&self) -> EventHandler<Event<MouseData>> {
        self.on_click
    }
}

pub(super) fn use_override_key_handlers(
    is_focus_target: bool,
    on_activate: EventHandler<()>,
) -> OverrideKeyHandlers {
    let focus = use_focus_coordinator();
    let on_mounted = EventHandler::new(move |event: Event<MountedData>| {
        if is_focus_target {
            let handle: Rc<MountedData> = event.data();
            focus.set_override_key_handle(Some(handle));
        }
    });
    use_drop(move || {
        if is_focus_target {
            focus.set_override_key_handle(None);
        }
    });
    let on_click = EventHandler::new(move |_event: Event<MouseData>| {
        on_activate.call(());
    });
    OverrideKeyHandlers {
        on_mounted,
        on_click,
    }
}
