use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex",
        "shrink-0",
        "items-stretch",
        "gap-1",
        "px-4",
        "pt-2",
        "pb-2",
        "h-28",
        "text-2xs",
    ],
    tablet: tw![
        "tablet:h-36",
        "tablet:text-base",
    ],
    laptop: tw![
        "laptop:h-48",
        "laptop:text-xl",
    ],
    desktop: tw![
        "desktop:h-36",
        "desktop:text-xl",
        "desktop:px-0",
        "desktop:pt-0",
        "desktop:pb-0",
    ],
    qhd: tw![
        "qhd:h-36",
        "qhd:text-xl",
        "qhd:px-0",
        "qhd:pt-0",
        "qhd:pb-0",
    ],
    uhd: tw![
        "uhd:h-36",
        "uhd:text-2xl",
        "uhd:px-0",
        "uhd:pt-0",
        "uhd:pb-0",
    ],
}
