use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-row",
        "flex-nowrap",
        "items-center",
        "gap-2",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:w-full",
    ],
    tablet: tw![
        "tablet:w-full",
    ],
}
