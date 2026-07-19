use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "justify-center",
        "h-full",
        "min-w-0",
        "shrink-0",
        "snap-start",
        "py-1.5",
    ],
    tablet: tw![
        "tablet:w-full",
        "tablet:max-w-4xl",
        "tablet:self-center",
    ],
}
