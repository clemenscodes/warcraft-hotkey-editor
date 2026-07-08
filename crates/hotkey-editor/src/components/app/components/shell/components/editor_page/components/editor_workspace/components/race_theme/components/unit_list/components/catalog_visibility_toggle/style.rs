use tw_macro::tw;
// The No-abilities / All-variants visibility toggle group. A side-by-side pair; the
// child buttons are tall on the sidebar and shorter on mobile.

classes! {
    base: tw![
        "flex",
        "flex-row",
        "gap-2",
        "[&>button]:min-h-[6.7rem]!",
    ],
    mobile: tw!["mobile:[&>button]:min-h-14!"],
}
