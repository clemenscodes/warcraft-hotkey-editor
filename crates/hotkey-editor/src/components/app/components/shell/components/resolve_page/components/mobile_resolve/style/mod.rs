use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "flex-1",
        "min-h-0",
        "min-w-0",
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
