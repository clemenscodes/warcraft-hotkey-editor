use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-[1_1_0]",
        "max-w-[50%]",
        "flex-col",
        "items-center",
        "gap-3",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:gap-[0.75em]",
    ],
    tablet: tw![
        "tablet:gap-[0.75em]",
    ],
    laptop: tw![
        "laptop:gap-[0.75em]",
    ],
    desktop: tw![
        "desktop:gap-[0.75em]",
    ],
    qhd: tw![
        "qhd:gap-[0.75em]",
    ],
    uhd: tw![
        "uhd:gap-[0.75em]",
    ],
}
