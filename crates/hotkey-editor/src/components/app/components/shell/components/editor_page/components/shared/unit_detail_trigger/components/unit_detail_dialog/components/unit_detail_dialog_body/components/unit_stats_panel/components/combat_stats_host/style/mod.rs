use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "min-w-0",
        "empty:hidden",
        "[grid-area:combat]",
    ],
    mobile: tw![
        "mobile:order-3",
    ],
    tablet: tw![
        "tablet:order-3",
    ],
    laptop: tw![
        "laptop:order-3",
    ],
}
