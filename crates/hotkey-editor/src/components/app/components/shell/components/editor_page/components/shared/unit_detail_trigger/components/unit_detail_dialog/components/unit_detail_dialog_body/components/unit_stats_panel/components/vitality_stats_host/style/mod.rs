use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "min-w-0",
        "empty:hidden",
        "[grid-area:vitality]",
    ],
    mobile: tw![
        "mobile:order-1",
    ],
    tablet: tw![
        "tablet:order-1",
    ],
    laptop: tw![
        "laptop:order-1",
    ],
}
