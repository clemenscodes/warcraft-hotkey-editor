use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-2",
        "[&>button]:min-h-[6.7rem]!",
    ],
    mobile: tw![
        "mobile:flex-row",
        "mobile:[&>button]:min-h-14!",
    ],
}
