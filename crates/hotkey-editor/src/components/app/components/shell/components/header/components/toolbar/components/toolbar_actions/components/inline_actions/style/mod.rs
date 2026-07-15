use tw_macro::tw;

classes! {
    base: tw![
        "hidden",
        "flex-row",
        "items-center",
        "justify-end",
        "gap-1",
        "min-w-0",
        "h-full",
    ],
    desktop: tw![
        "desktop:flex",
    ],
    qhd: tw![
        "qhd:flex",
    ],
    uhd: tw![
        "uhd:flex",
    ],
}
