use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-2",
        "min-w-0",
        "flex-1",
    ],
    mobile: tw![
        "mobile:items-start",
        "mobile:text-left",
        "mobile:gap-1",
        "mobile:overflow-hidden",
    ],
    tablet: tw![
        "tablet:items-start",
        "tablet:text-left",
        "tablet:gap-1",
        "tablet:overflow-hidden",
    ],
}
