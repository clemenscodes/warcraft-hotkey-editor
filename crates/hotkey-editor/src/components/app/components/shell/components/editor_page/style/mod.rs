use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "gap-2",
        "flex-[1_1_0]",
        "min-h-0",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:flex-none",
    ],
    tablet: tw![
        "tablet:flex-none",
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
