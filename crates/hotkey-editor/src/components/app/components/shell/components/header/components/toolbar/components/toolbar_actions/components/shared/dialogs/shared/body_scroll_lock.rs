use dioxus::prelude::*;
use std::cell::Cell;
use wasm_bindgen::JsCast;

thread_local! {
    static OPEN_DIALOG_COUNT: Cell<u32> = const { Cell::new(0) };
}

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

fn scroll_root() -> Option<web_sys::HtmlElement> {
    let window = web_sys::window()?;
    let document = window.document()?;
    let root = document.document_element()?;
    root.dyn_into::<web_sys::HtmlElement>().ok()
}
