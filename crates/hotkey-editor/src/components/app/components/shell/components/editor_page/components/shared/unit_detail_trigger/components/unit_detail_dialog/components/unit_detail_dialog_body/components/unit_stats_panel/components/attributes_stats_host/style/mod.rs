use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "min-w-0",
        "empty:hidden",
        "[grid-area:attributes]",
    ],
    mobile: tw![
        "mobile:order-2",
    ],
    tablet: tw![
        "tablet:order-2",
    ],
    laptop: tw![
        "laptop:order-2",
    ],
}
