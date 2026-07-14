use tw_macro::tw;
// Layout-neutral grouping wrapper: `contents` dissolves this row's own box so the menu-item
// button lays out as a direct child of the drawer body, like the other rows. The wrapper
// exists only to carry this row's identity and group the item with the dialog it owns.

classes! {
    base: tw![
        "contents",
    ],
}
