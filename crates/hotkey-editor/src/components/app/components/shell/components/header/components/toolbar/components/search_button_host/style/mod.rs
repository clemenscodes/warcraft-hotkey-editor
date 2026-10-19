use tw_macro::tw;

classes! {
    base: tw![
        "inline-flex",
        "shrink-0",
        "@container",
        "h-full",
        "aspect-square",
    ],
    desktop: tw![
        "desktop:hidden",
    ],
    qhd: tw![
        "qhd:hidden",
    ],
    uhd: tw![
        "uhd:hidden",
    ],
}
