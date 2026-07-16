use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-row",
        "flex-nowrap",
        "items-start",
        "gap-8",
    ],
    mobile: tw![
        "mobile:flex-col",
        "mobile:items-stretch",
        "mobile:gap-3",
    ],
    tablet: tw![
        "tablet:gap-2",
    ],
}
