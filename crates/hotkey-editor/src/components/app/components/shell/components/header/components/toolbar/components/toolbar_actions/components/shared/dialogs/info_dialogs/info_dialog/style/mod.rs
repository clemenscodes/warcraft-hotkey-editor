use tw_macro::tw;
// A dialog base is a data seam, not a box: it owns no layout of its own. The
// `WarcraftDialog` overlay it renders positions itself fixed, so this root is
// `contents` — a layout-neutral grouping wrapper carrying only the identity class.

classes! {
    base: tw![
        "contents",
    ],
}
