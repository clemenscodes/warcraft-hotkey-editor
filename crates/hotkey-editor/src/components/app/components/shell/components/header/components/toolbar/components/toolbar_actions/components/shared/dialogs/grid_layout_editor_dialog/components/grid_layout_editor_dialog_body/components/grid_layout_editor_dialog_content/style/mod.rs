use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "gap-16",
    ],
    mobile: tw![
        "mobile:flex-1",
        "mobile:justify-start",
        "mobile:gap-5",
    ],
    tablet: tw![
        "tablet:flex-1",
        "tablet:justify-center-safe",
        "tablet:gap-5",
    ],
    laptop: tw![
        "laptop:flex-1",
        "laptop:justify-center-safe",
        "laptop:gap-5",
    ],
    desktop: tw![
        "desktop:flex-1",
        "desktop:justify-center-safe",
        "desktop:gap-5",
    ],
    qhd: tw![
        "qhd:flex-1",
        "qhd:justify-center-safe",
        "qhd:gap-7",
    ],
    uhd: tw![
        "uhd:flex-1",
        "uhd:justify-center-safe",
        "uhd:gap-8.5",
    ],
}
