use tw_macro::tw;
// The dialog body region wraps the shared position-picker scroll body, which owns the
// scroll region itself, so this component adds no box: its root is `contents` — a
// layout-neutral grouping wrapper carrying only the identity class — and the shared body's
// `flex-1` lays out directly against the dialog content box.

classes! {
    base: tw![
        "contents",
    ],
}
