use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-2",
    ],
    mobile: tw![
        "mobile:flex-row",
        "mobile:flex-nowrap",
        "mobile:w-max",
        "mobile:min-w-full",
        "mobile:items-stretch",
        "mobile:h-full",
        "mobile:px-1.5",
        "mobile:py-0",
    ],
    tablet: tw![
        "tablet:flex-row",
        "tablet:flex-nowrap",
        "tablet:w-max",
        "tablet:min-w-full",
        "tablet:items-stretch",
        "tablet:h-full",
        "tablet:px-1.5",
        "tablet:py-0",
    ],
    laptop: tw![
        "laptop:flex-row",
        "laptop:flex-nowrap",
        "laptop:w-max",
        "laptop:min-w-full",
        "laptop:items-stretch",
        "laptop:h-full",
        "laptop:px-1.5",
        "laptop:py-0",
    ],
    desktop: tw![
        "desktop:grid",
        "desktop:grid-cols-2",
        "desktop:content-start",
    ],
    qhd: tw![
        "qhd:grid",
        "qhd:grid-cols-3",
        "qhd:content-start",
    ],
    uhd: tw![
        "uhd:grid",
        "uhd:grid-cols-4",
        "uhd:content-start",
    ],
}
