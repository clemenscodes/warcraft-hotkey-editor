use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-row",
        "items-center",
        "gap-6",
        "w-full",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:flex-col",
        "mobile:items-start",
        "mobile:gap-2",
    ],
    tablet: tw![
        "tablet:flex-col",
        "tablet:items-start",
        "tablet:gap-2",
    ],
}
