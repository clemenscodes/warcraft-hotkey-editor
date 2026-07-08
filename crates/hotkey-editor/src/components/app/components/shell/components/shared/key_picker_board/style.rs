use tw_macro::tw;
// Its own query container: keys size themselves in `cqi` off this board's width (not
// the viewport), so they scale correctly whether the dialog is at its base, tablet,
// or mobile width. Columns sit side by side, bottom-aligned, and centered as a group.

classes! {
    base: tw![
        "flex",
        "flex-row",
        "items-end",
        "justify-center",
        "gap-3",
        "w-full",
        "@container",
    ],
    mobile: tw![
        "mobile:flex-col",
        "mobile:items-center",
        "mobile:gap-1.5",
    ],
}
