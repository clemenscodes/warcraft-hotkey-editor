use tw_macro::tw;
// The layout editor renders two self-positioning overlays (the `WarcraftDialog` and the
// nested key picker), so this root owns no box: `contents` is a layout-neutral grouping
// wrapper carrying only the identity class.

classes! {
    base: tw![
        "contents",
    ],
}
