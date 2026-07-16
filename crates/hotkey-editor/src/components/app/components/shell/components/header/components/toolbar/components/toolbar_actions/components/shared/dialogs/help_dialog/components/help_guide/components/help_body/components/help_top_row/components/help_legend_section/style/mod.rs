use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "gap-5",
        "flex-1",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:flex-none",
        "mobile:w-full",
    ],
    tablet: tw![
        "tablet:flex-none",
        "tablet:w-full",
    ],
}
