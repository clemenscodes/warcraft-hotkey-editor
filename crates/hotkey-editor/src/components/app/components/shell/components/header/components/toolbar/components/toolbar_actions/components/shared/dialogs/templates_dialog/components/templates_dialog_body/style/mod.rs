use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex-1",
        "min-h-0",
        "flex",
        "flex-col",
        "gap-6",
        "pt-10",
        "px-12",
        "pb-10",
        "overflow-y-auto",
    ],
    mobile: tw![
        "mobile:pt-5",
        "mobile:px-4",
        "mobile:pb-6",
    ],
    tablet: tw![
        "tablet:pt-5",
        "tablet:px-4",
        "tablet:pb-6",
    ],
    laptop: tw![
        "laptop:pt-5",
        "laptop:px-4",
        "laptop:pb-6",
    ],
    desktop: tw![
        "desktop:pt-5",
        "desktop:px-4",
        "desktop:pb-6",
    ],
    qhd: tw![
        "qhd:gap-8.5",
        "qhd:pt-7",
        "qhd:px-5.5",
        "qhd:pb-8.5",
    ],
    uhd: tw![
        "uhd:gap-10.25",
        "uhd:pt-8.5",
        "uhd:px-6.75",
        "uhd:pb-10.25",
    ],
}
