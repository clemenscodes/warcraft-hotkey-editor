use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex",
        "items-stretch",
        "flex-none",
        "gap-10",
        "min-h-36",
    ],
    mobile: tw![
        "mobile:flex-col",
        "mobile:min-h-0",
        "mobile:gap-2.5",
    ],
    tablet: tw![
        "tablet:flex-col",
        "tablet:min-h-0",
        "tablet:gap-2.5",
    ],
    laptop: tw![
        "laptop:flex-col",
        "laptop:min-h-0",
        "laptop:gap-2.5",
    ],
    desktop: tw![
        "desktop:col-start-2",
        "desktop:row-start-1",
        "desktop:gap-6",
    ],
    qhd: tw![
        "qhd:col-start-2",
        "qhd:row-start-1",
        "qhd:gap-6",
    ],
    uhd: tw![
        "uhd:col-start-2",
        "uhd:row-start-1",
        "uhd:gap-6",
    ],
}
