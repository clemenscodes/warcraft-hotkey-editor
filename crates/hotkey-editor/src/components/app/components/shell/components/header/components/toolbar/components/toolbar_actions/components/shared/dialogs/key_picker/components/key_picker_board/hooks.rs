#[cfg(target_arch = "wasm32")]
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

/// Off the browser there is no document to focus, so the hook does nothing.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn use_board_focus() {}
