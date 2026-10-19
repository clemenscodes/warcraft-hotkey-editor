use tw_macro::tw;

// Laid out like the grid layout dialog: the explainer and the grid sit
// together in the middle of the dialog, the grid as wide as the layout grid.
classes! {
    base: tw![
        "@container",
        "flex-1",
        "min-h-0",
        "flex",
        "flex-col",
        "items-center",
        "gap-6",
        "overflow-y-auto",
        "pt-10",
        "px-12",
        "pb-10",
    ],
    mobile: tw![
        "mobile:justify-start",
        "mobile:gap-5",
        "mobile:pt-5",
        "mobile:px-4",
        "mobile:pb-6",
    ],
    tablet: tw![
        "tablet:justify-center-safe",
        "tablet:gap-5",
        "tablet:pt-5",
        "tablet:px-4",
        "tablet:pb-6",
    ],
    laptop: tw![
        "laptop:justify-center-safe",
        "laptop:gap-5",
        "laptop:pt-5",
        "laptop:px-4",
        "laptop:pb-6",
    ],
    desktop: tw![
        "desktop:justify-center-safe",
        "desktop:gap-5",
        "desktop:pt-5",
        "desktop:px-4",
        "desktop:pb-6",
    ],
    qhd: tw![
        "qhd:justify-center-safe",
        "qhd:gap-7",
        "qhd:pt-7",
        "qhd:px-5.5",
        "qhd:pb-8.5",
    ],
    uhd: tw![
        "uhd:justify-center-safe",
        "uhd:gap-8.5",
        "uhd:pt-8.5",
        "uhd:px-6.75",
        "uhd:pb-10.25",
    ],
}
