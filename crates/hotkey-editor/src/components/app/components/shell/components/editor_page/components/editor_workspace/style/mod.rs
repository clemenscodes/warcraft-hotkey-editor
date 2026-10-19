use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "relative",
        "grid",
        "grid-cols-[minmax(0,1fr)]",
        "flex-[1_1_0]",
        "items-stretch",
        "min-h-0",
        "gap-4",
    ],
    mobile: tw![
        "mobile:flex-none",
    ],
    tablet: tw![
        "tablet:flex-none",
    ],
    laptop: tw![
        "laptop:flex-none",
    ],
    desktop: tw![
        "desktop:contents",
    ],
    qhd: tw![
        "qhd:contents",
    ],
    uhd: tw![
        "uhd:contents",
    ],
}
