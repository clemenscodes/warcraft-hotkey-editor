use dioxus::prelude::*;

/// Keeps the board keyboard-ready on every open. The keydown handler only fires
/// while focus sits inside the board, but `autofocus` and Dioxus `set_focus` both
/// proved unreliable for this portal-mounted content: the dialog resets focus to
/// `document.body` a tick after mount, so the keyboard went dead after the first
/// reopen. This defers one tick past that reset, then focuses the board itself.
#[cfg(target_arch = "wasm32")]
pub(super) fn use_board_focus() {
    use_effect(move || {
        spawn(async move {
            use wasm_bindgen::JsCast;
            gloo_timers::future::TimeoutFuture::new(0).await;
            let Some(document) = web_sys::window().and_then(|window| window.document()) else {
                return;
            };
            let Some(node) = document.query_selector(".key-picker-board").ok().flatten() else {
                return;
            };
            if let Some(focusable) = node.dyn_ref::<web_sys::HtmlElement>() {
                let _ = focusable.focus();
            }
        });
    });
}

/// A focus-independent fallback for the same gap `use_board_focus` works around.
/// Between mount and that deferred focus, focus sits on `document.body`, so a letter
/// pressed in the gap never reaches the board's own `onkeydown` and is dropped —
/// invisible to a human, but a genuinely lost keypress (and the source of a flaky
/// test). This document-level keydown listener, mounted only while the board is open
/// and removed on unmount, routes a letter into the same `pending_key` signal the
/// picker resolves. It fires only while the board is NOT the active element; once
/// focus lands on the board its own handler takes over, so the two never double-pick.
/// Mirrors the proven document-listener pattern in `services/undo`.
#[cfg(target_arch = "wasm32")]
pub(super) fn use_board_keyboard(pending_key: Signal<Option<String>>) {
    use std::rc::Rc;
    use wasm_bindgen::JsCast;
    use wasm_bindgen::closure::Closure;
    let listener = use_hook(|| {
        let mut pending_key = pending_key;
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
                let mut characters = key.chars();
                let is_single_letter = characters
                    .next()
                    .map(|character| character.is_ascii_alphabetic())
                    .unwrap_or(false)
                    && characters.next().is_none();
                if !is_single_letter {
                    return;
                }
                event.prevent_default();
                pending_key.set(Some(key));
            },
        );
        if let Some(document) = web_sys::window().and_then(|window| window.document()) {
            let target = closure.as_ref().unchecked_ref();
            let _ = document.add_event_listener_with_callback("keydown", target);
        }
        Rc::new(closure)
    });
    use_drop(move || {
        if let Some(document) = web_sys::window().and_then(|window| window.document()) {
            let js_value: &wasm_bindgen::JsValue = listener.as_ref().as_ref();
            let target = js_value.unchecked_ref();
            let _ = document.remove_event_listener_with_callback("keydown", target);
        }
    });
}

/// Off the browser there is no document to focus, so the hook does nothing.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn use_board_focus() {}

/// Off the browser there is no document to listen on, so the hook does nothing.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn use_board_keyboard(_pending_key: Signal<Option<String>>) {}
