use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "flex-1",
        "min-h-0",
        "flex",
        "flex-col",
        "gap-4",
        "pt-6",
        "px-6",
        "pb-6",
    ],
    mobile: tw![
        "mobile:gap-3",
        "mobile:pt-4",
        "mobile:px-4",
        "mobile:pb-4",
    ],
    tablet: tw![
        "tablet:gap-3",
        "tablet:pt-4",
        "tablet:px-4",
        "tablet:pb-4",
    ],
    laptop: tw![
        "laptop:gap-3",
        "laptop:pt-4",
        "laptop:px-4",
        "laptop:pb-4",
    ],
    desktop: tw![
        "desktop:gap-3.5",
        "desktop:p-0",
    ],
    qhd: tw![
        "qhd:gap-5.25",
        "qhd:p-0",
    ],
    uhd: tw![
        "uhd:gap-5.5",
        "uhd:p-0",
    ],
}
