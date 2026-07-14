use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-row",
        "gap-2",
        "[&>button]:min-h-[6.7rem]!",
    ],
    mobile: tw![
        "mobile:[&>button]:min-h-14!",
    ],
}
