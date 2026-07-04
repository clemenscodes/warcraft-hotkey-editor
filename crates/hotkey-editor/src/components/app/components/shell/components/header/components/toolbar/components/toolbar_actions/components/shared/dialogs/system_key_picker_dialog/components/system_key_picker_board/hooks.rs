#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;

/// Keeps the board keyboard-ready on every open. The portal-mounted content has
/// its focus reset to `document.body` a tick after mount, so `autofocus` only ever
/// worked on the first open. This defers past that reset and focuses the board
/// itself, matching the shared `KeyPicker` board.
#[cfg(target_arch = "wasm32")]
pub(super) fn use_board_focus() {
    use_effect(move || {
        spawn(async move {
            use wasm_bindgen::JsCast;
            gloo_timers::future::TimeoutFuture::new(0).await;
            let Some(document) = web_sys::window().and_then(|window| window.document()) else {
                return;
            };
            let Some(node) = document
                .query_selector(".system-key-picker-board")
                .ok()
                .flatten()
            else {
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
