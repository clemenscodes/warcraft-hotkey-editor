use dioxus::prelude::*;

/// Locks body scroll while a dialog is open. Every dialog owns this side effect
/// so its markup never touches the document: it watches the open signal and
/// writes `overflow: hidden` onto the body, clearing it again on close. The
/// unmount cleanup mirrors the close branch so a dialog that is unmounted while
/// open (the conditionally-mounted ones tear down their whole subtree instead of
/// flipping the flag to false) still releases the lock. Shared by every dialog
/// shell, so it lives beside the shared `DialogHeader`.
pub(crate) fn use_body_scroll_lock(open: Signal<bool>) {
    use_effect(move || {
        let is_open = open();
        let body = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.body());
        let Some(body) = body else {
            return;
        };
        let style = body.style();
        if is_open {
            let _ = style.set_property("overflow", "hidden");
            let _ = style.set_property("overscroll-behavior", "contain");
        } else {
            let _ = style.remove_property("overflow");
            let _ = style.remove_property("overscroll-behavior");
        }
    });
    use_drop(move || {
        let body = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.body());
        let Some(body) = body else {
            return;
        };
        let style = body.style();
        let _ = style.remove_property("overflow");
        let _ = style.remove_property("overscroll-behavior");
    });
}
