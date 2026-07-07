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
        "mobile:gap-10",
    ],
    tablet: tw![
        "tablet:flex-col",
        "tablet:gap-10",
    ],
}
