use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "min-w-0",
        "shrink-0",
        "snap-start",
        "snap-always",
        "py-1.5",
    ],
    tablet: tw![
        "tablet:w-full",
        "tablet:max-w-4xl",
        "tablet:self-center",
    ],
}
