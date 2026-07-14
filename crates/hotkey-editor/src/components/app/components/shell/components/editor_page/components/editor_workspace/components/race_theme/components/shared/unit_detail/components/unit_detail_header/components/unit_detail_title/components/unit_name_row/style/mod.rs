use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "items-center",
        "justify-between",
        "gap-3.5",
        "min-h-0",
    ],
    mobile: tw![
        "mobile:flex-row",
        "mobile:items-center",
        "mobile:gap-2",
        "mobile:w-full",
        "mobile:min-w-0",
    ],
    tablet: tw![
        "tablet:flex-row",
        "tablet:items-center",
        "tablet:gap-2",
        "tablet:w-full",
        "tablet:min-w-0",
    ],
}
