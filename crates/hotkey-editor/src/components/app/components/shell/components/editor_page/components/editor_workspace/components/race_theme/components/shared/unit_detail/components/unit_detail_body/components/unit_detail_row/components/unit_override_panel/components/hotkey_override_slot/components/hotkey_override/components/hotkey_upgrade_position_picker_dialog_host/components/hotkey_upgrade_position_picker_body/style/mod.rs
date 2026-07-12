use tw_macro::tw;
// The dialog's scroll-body region wrapper: layout-neutral so the shared position-picker
// body lays out as a direct flex child of the dialog content box. `contents` adds only
// the identity class, no box of its own.

classes! {
    base: tw![
        "contents",
    ],
}
