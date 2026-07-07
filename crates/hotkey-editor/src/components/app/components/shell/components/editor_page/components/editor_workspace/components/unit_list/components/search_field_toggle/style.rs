use tw_macro::tw;
// The Unit/Ability search-field toggle group. A stacked pair that becomes a side-by-
// side row on small screens; the child buttons are tall on the sidebar and shorter
// on mobile.

classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-2",
        "mb-2",
        "[&>button]:min-h-[6.7rem]!",
    ],
    mobile: tw![
        "mobile:flex-row",
        "mobile:[&>button]:min-h-14!",
    ],
}
