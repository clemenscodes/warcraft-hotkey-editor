use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::deferred_focus::use_deferred_focus;
use super::model::KeyPickerBoardHostModel;
use crate::components::app::components::shell::components::shared::key_picker_board::BrowserKeyEvent;
use dioxus::prelude::*;
use std::rc::Rc;
use warcraft_keybinds::KeyCode;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

pub(super) fn use_board_keyboard(props: &KeyPickerBoardHostModel) {
    use_deferred_focus(".key-picker-board");
    use_focus_gap_keyboard(props);
}

#[derive(Clone, Copy, PartialEq)]
enum PendingKeyPress {
    Pick(KeyCode),
    Dismiss,
}

fn use_focus_gap_keyboard(props: &KeyPickerBoardHostModel) {
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
