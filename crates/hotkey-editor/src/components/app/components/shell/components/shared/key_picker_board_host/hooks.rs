use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::deferred_focus::use_deferred_focus;
use crate::components::app::components::shell::components::shared::key_picker_board::BrowserKeyEvent;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyPickerBoardProps;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_keybinds::KeyCode;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

/// Wires the picker's "works anywhere" concerns: focuses the board a tick after mount
/// (see [`use_deferred_focus`]) and a document-level keydown fallback for the brief gap
/// before that focus lands — when a keypress would otherwise reach `document.body` and
/// be dropped. Both live on the picker, not on any dialog, so the board behaves
/// identically wherever it is placed.
pub(super) fn use_board_keyboard(props: &KeyPickerBoardProps) {
    use_deferred_focus(".key-picker-board");
    use_focus_gap_keyboard(props);
}

/// What a focus-gap keypress resolves to before the effect can act on it: an offered
/// key to pick, or an Escape dismiss. Escape is a dismiss even though it is itself a
/// bindable key — a physical Escape cancels the picker; the Escape cap is bound by
/// clicking it.
#[derive(Clone, Copy, PartialEq)]
enum PendingKeyPress {
    Pick(KeyCode),
    Dismiss,
}

/// The focus-gap fallback: while focus still sits on `document.body`, a physical
/// keypress never reaches the board's own `onkeydown`, so a document-level listener
/// resolves it — Escape dismisses, any other press is matched against the offered keys
/// — and parks the outcome in a signal the effect drains into the pick / close handler.
/// It fires only while the board is NOT the active element; once focus lands, the
/// board's own handler takes over, so the two never double-fire. Off the browser
/// `web_sys::window()` is `None`, so the listener is never installed and the hook is a
/// no-op.
fn use_focus_gap_keyboard(props: &KeyPickerBoardProps) {
    let mut offered: Vec<KeyCode> = Vec::new();
    for column in &props.columns {
        let column_codes = column.pickable_codes();
        offered.extend(column_codes);
    }
    let mut pending = use_signal(|| Option::<PendingKeyPress>::None);
    let listener = use_hook(move || {
        let mut pending = pending;
        let offered = offered;
        let closure = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
            move |event: web_sys::KeyboardEvent| {
                let active_is_board = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.active_element())
                    .map(|active| active.matches(".key-picker-board").unwrap_or(false))
                    .unwrap_or(false);
                if active_is_board {
                    return;
                }
                let key = event.key();
                if key == "Escape" {
                    event.prevent_default();
                    pending.set(Some(PendingKeyPress::Dismiss));
                    return;
                }
                let code = event.code();
                let browser_event = BrowserKeyEvent::new(&key, &code);
                let Some(picked) = browser_event.pick_among(&offered) else {
                    return;
                };
                event.prevent_default();
                pending.set(Some(PendingKeyPress::Pick(picked)));
            },
        );
        if let Some(document) = web_sys::window().and_then(|window| window.document()) {
            let target = closure.as_ref().unchecked_ref();
            let _ = document.add_event_listener_with_callback("keydown", target);
        }
        Rc::new(closure)
    });
    let on_pick = props.on_pick;
    let on_close = props.on_close;
    use_effect(move || {
        let Some(press) = pending() else {
            return;
        };
        pending.set(None);
        match press {
            PendingKeyPress::Pick(code) => on_pick.call(code),
            PendingKeyPress::Dismiss => on_close.call(()),
        }
    });
    use_drop(move || {
        if let Some(document) = web_sys::window().and_then(|window| window.document()) {
            let js_value: &wasm_bindgen::JsValue = listener.as_ref().as_ref();
            let target = js_value.unchecked_ref();
            let _ = document.remove_event_listener_with_callback("keydown", target);
        }
    });
}
