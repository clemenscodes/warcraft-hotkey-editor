use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "w-full",
    ],
    mobile: tw![
        "mobile:flex-1",
        "mobile:justify-center-safe",
    ],
    tablet: tw![
        "tablet:flex-1",
        "tablet:justify-center-safe",
        "tablet:p-6",
    ],
    laptop: tw![
        "laptop:flex-1",
        "laptop:justify-center-safe",
        "laptop:p-6",
    ],
    desktop: tw![
        "desktop:flex-1",
        "desktop:justify-center-safe",
        "desktop:pt-5",
        "desktop:px-4",
        "desktop:pb-6",
    ],
    qhd: tw![
        "qhd:flex-1",
        "qhd:justify-center-safe",
        "qhd:pt-7",
        "qhd:px-5.5",
        "qhd:pb-8.5",
    ],
    uhd: tw![
        "uhd:flex-1",
        "uhd:justify-center-safe",
        "uhd:pt-8.5",
        "uhd:px-6.75",
        "uhd:pb-10.25",
    ],
}
