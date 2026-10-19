use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use std::cell::Cell;
use std::rc::Rc;

/// Opens and closes a dialog from the button that shows it. A press outside an
/// open dialog dismisses it while the press is still being dispatched, before
/// the button sees it, so the toggle remembers when the dialog last closed: a
/// press that began before that close still found the dialog open, and its click
/// closes the dialog instead of opening it again.
#[derive(Clone)]
pub(crate) struct DialogToggle {
    open: Signal<bool>,
    open_at_press: Rc<Cell<Option<bool>>>,
    closed_at: Rc<Cell<f64>>,
}

impl DialogToggle {
    /// Follows the dialog's own open changes, noting when it closes.
    pub(crate) fn on_open_change(&self) -> Callback<bool> {
        let mut open = self.open;
        let closed_at = self.closed_at.clone();
        Callback::new(move |is_open: bool| {
            if !is_open {
                closed_at.set(now());
            }
            open.set(is_open);
        })
    }

    /// Remembers whether the dialog was open as the press on its host began.
    pub(crate) fn onpointerdown(&self) -> EventHandler<PointerEvent> {
        let open = self.open;
        let open_at_press = self.open_at_press.clone();
        let closed_at = self.closed_at.clone();
        EventHandler::new(move |event: PointerEvent| {
            let pressed_at = event
                .data()
                .try_as_web_event()
                .map(|web_event| web_event.time_stamp())
                .unwrap_or(f64::INFINITY);
            let closed_by_this_press = closed_at.get() >= pressed_at;
            open_at_press.set(Some(*open.peek() || closed_by_this_press));
        })
    }

    /// Toggles the dialog from what it was when the press began. A click from the
    /// keyboard has no press, so it toggles from what the dialog is now.
    pub(crate) fn onclick(&self) -> EventHandler<MouseEvent> {
        let mut open = self.open;
        let open_at_press = self.open_at_press.clone();
        EventHandler::new(move |_event: MouseEvent| {
            let was_open = open_at_press.take().unwrap_or(*open.peek());
            open.set(!was_open);
        })
    }
}

// The same clock event time stamps are measured on.
fn now() -> f64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map(|performance| performance.now())
        .unwrap_or(f64::NEG_INFINITY)
}

pub(crate) fn use_dialog_toggle(open: Signal<bool>) -> DialogToggle {
    let open_at_press = use_hook(|| Rc::new(Cell::new(None)));
    let closed_at = use_hook(|| Rc::new(Cell::new(f64::NEG_INFINITY)));
    DialogToggle {
        open,
        open_at_press,
        closed_at,
    }
}
