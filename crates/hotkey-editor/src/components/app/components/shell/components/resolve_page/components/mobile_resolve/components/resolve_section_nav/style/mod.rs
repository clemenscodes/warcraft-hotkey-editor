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
    ],
    qhd: tw![
        "qhd:h-36",
        "qhd:text-xl",
    ],
    uhd: tw![
        "uhd:h-36",
        "uhd:text-2xl",
    ],
}
