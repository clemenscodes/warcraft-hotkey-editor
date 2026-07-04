use dioxus::prelude::*;

/// Locks body scroll while the dialog is open. The shell owns this side effect
/// so the markup never touches the document: it watches the open signal and
/// writes `overflow: hidden` onto the body, clearing it again on close. This is
/// the shell's one hook, the only line of the base body that is not pure RSX.
pub(super) fn use_body_scroll_lock(open: Signal<bool>) {
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
}
