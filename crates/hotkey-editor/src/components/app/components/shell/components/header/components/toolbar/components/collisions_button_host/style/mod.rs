use tw_macro::tw;

classes! {
    base: tw![
        "inline-flex",
        "shrink-0",
        "@container",
        "h-full",
        "aspect-square",
    ],
    mobile: tw![
        "mobile:w-9",
        "mobile:h-9",
    ],
    tablet: tw![
        "tablet:w-9",
        "tablet:h-9",
    ],
}
