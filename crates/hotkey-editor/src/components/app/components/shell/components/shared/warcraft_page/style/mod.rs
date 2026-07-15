use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "gap-2",
        "px-4",
        "flex-[1_1_0]",
        "min-h-0",
        "min-w-0",
    ],
    laptop: tw![
        "laptop:gap-6",
    ],
    desktop: tw![
        "desktop:gap-6",
    ],
    qhd: tw![
        "qhd:gap-6",
    ],
    uhd: tw![
        "uhd:gap-6",
    ],
}
