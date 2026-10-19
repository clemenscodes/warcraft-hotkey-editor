use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "min-w-0",
        "shrink-0",
        "py-1.5",
    ],
    tablet: tw![
        "tablet:min-h-0",
        "tablet:w-full",
    ],
    laptop: tw![
        "laptop:min-h-0",
        "laptop:w-full",
    ],
    desktop: tw![
        "desktop:min-h-0",
        "desktop:w-full",
    ],
    qhd: tw![
        "qhd:min-h-0",
        "qhd:w-full",
    ],
    uhd: tw![
        "uhd:min-h-0",
        "uhd:w-full",
    ],
}
