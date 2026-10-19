use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "gap-10",
        "items-stretch",
        "flex-none",
        "min-h-0",
    ],
    desktop: tw![
        "desktop:flex-[1_1_0]",
    ],
    qhd: tw![
        "qhd:flex-[1_1_0]",
    ],
    uhd: tw![
        "uhd:flex-[1_1_0]",
    ],
}
