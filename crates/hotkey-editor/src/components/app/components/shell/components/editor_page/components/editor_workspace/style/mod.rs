use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "relative",
        "flex",
        "flex-col",
        "flex-[1_1_0]",
        "min-h-0",
        "min-w-0",
        "gap-4",
    ],
    mobile: tw![
        "mobile:flex-none",
    ],
    tablet: tw![
        "tablet:flex-none",
    ],
}
