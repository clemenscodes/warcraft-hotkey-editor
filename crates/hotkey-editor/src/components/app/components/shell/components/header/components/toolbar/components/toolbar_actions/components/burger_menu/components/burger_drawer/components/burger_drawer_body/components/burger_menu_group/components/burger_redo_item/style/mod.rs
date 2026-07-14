use tw_macro::tw;
// Layout-neutral grouping wrapper: `contents` dissolves this row's own box so the menu-item
// button lays out as a direct child of the drawer body, exactly like the data-driven rows.
// The wrapper exists only to carry this row's identity.

classes! {
    base: tw![
        "contents",
    ],
}
