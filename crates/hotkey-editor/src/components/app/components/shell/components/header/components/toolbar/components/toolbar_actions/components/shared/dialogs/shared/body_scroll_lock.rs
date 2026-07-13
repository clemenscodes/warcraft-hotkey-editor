use dioxus::prelude::*;
use std::cell::Cell;
use wasm_bindgen::JsCast;

thread_local! {
    /// Count of currently-open dialogs. Dialogs mount only while open, so a mounted
    /// dialog is an open one — but the app allows one dialog to nest a second (the grid
    /// layout editor opens a key picker), so at a given moment the count is zero, one, or
    /// two. The page is locked the moment the count rises to one and released only when it
    /// falls back to zero, so a nested dialog closing never unlocks the page while its
    /// parent dialog is still open. The count is owned here, not in a context or service:
    /// scroll-lock is purely a function of how many dialogs are open, and wasm is
    /// single-threaded, so a thread-local is the whole mechanism.
    static OPEN_DIALOG_COUNT: Cell<u32> = const { Cell::new(0) };
}

/// Locks page scroll while any dialog is open. Every dialog shell calls this; it counts
/// the open dialogs and locks the document's root element while that count is above zero.
/// The root is what actually scrolls: on mobile the viewport scrolls the root element, so
/// locking only `body` leaves the page free to move behind the dialog. Shared by every
/// dialog shell, so it lives beside the shared `DialogHeader`.
pub(crate) fn use_body_scroll_lock() {
    use_effect(move || {
        let is_first_open_dialog = OPEN_DIALOG_COUNT.with(|open_dialog_count| {
            let raised_count = open_dialog_count.get() + 1;
            open_dialog_count.set(raised_count);
            raised_count == 1
        });
        if is_first_open_dialog && let Some(root) = scroll_root() {
            let style = root.style();
            let _ = style.set_property("overflow", "hidden");
            let _ = style.set_property("overscroll-behavior", "contain");
        }
    });
    use_drop(move || {
        let was_last_open_dialog = OPEN_DIALOG_COUNT.with(|open_dialog_count| {
            let lowered_count = open_dialog_count.get().saturating_sub(1);
            open_dialog_count.set(lowered_count);
            lowered_count == 0
        });
        if was_last_open_dialog && let Some(root) = scroll_root() {
            let style = root.style();
            let _ = style.remove_property("overflow");
            let _ = style.remove_property("overscroll-behavior");
        }
    });
}

/// The document's root (`<html>`) element as an `HtmlElement`, whose inline style
/// governs viewport scrolling. `None` off the browser or before the document exists.
fn scroll_root() -> Option<web_sys::HtmlElement> {
    let window = web_sys::window()?;
    let document = window.document()?;
    let root = document.document_element()?;
    root.dyn_into::<web_sys::HtmlElement>().ok()
}
