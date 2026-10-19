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
        "mobile:pt-6",
        "mobile:px-8",
        "mobile:pb-6",
    ],
    tablet: tw![
        "tablet:pt-6",
        "tablet:px-8",
        "tablet:pb-6",
    ],
    laptop: tw![
        "laptop:pt-6",
        "laptop:px-8",
        "laptop:pb-6",
    ],
    desktop: tw![
        "desktop:pt-6",
        "desktop:px-8",
        "desktop:pb-6",
    ],
    qhd: tw![
        "qhd:pt-8.5",
        "qhd:px-11.25",
        "qhd:pb-8.5",
    ],
    uhd: tw![
        "uhd:pt-10.25",
        "uhd:px-13.5",
        "uhd:pb-10.25",
    ],
}
