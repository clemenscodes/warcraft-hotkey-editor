use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "min-w-0",
        "empty:hidden",
        "[grid-area:defense]",
    ],
    mobile: tw![
        "mobile:order-4",
    ],
    tablet: tw![
        "tablet:order-4",
    ],
    laptop: tw![
        "laptop:order-4",
    ],
}
