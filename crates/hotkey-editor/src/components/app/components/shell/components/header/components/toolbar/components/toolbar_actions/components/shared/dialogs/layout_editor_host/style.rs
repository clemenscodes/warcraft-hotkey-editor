use tw_macro::tw;
// A dialog host is a data seam, not a box: it owns no layout of its own. Its overlay
// child positions itself fixed, so the host root is `contents` — a layout-neutral
// grouping wrapper carrying only the identity class, adding no box to the toolbar row.

classes! {
    base: tw![
        "contents",
    ],
}
