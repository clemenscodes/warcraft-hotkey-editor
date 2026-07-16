use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-row",
        "items-start",
        "gap-13",
    ],
    mobile: tw![
        "mobile:flex-col",
        "mobile:items-stretch",
        "mobile:gap-10",
    ],
    tablet: tw![
        "tablet:flex-col",
        "tablet:items-stretch",
        "tablet:gap-10",
    ],
}
