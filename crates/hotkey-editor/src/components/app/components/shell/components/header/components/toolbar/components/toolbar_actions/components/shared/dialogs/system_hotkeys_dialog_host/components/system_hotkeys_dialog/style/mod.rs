use tw_macro::tw;
// The dialog renders one self-positioning `WarcraftDialog` overlay, so its own root owns
// no box: `contents` is a layout-neutral grouping wrapper carrying only the identity class.

classes! {
    base: tw![
        "contents",
    ],
}
