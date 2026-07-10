use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "gap-16",
    ],
    mobile: tw![
        "mobile:justify-start",
        "mobile:gap-5",
    ],
    tablet: tw![
        "tablet:justify-start",
        "tablet:gap-5",
    ],
}
