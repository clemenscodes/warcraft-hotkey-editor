use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-2",
    ],
    tablet: tw![
        "tablet:flex-row",
        "tablet:flex-nowrap",
        "tablet:w-max",
        "tablet:min-w-full",
        "tablet:items-stretch",
        "tablet:h-full",
        "tablet:px-1.5",
        "tablet:py-0",
    ],
}
