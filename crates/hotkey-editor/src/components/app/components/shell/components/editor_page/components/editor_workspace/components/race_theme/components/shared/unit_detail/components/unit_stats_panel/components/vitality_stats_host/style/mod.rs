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
}
