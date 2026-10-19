use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "list-none",
        "p-0",
        "w-full",
        "max-w-440",
        "self-center",
        "flex",
        "flex-col",
    ],
    mobile: tw![
        "mobile:max-w-full",
        "mobile:px-3",
        "mobile:touch-pan-y",
    ],
    tablet: tw![
        "tablet:max-w-full",
        "tablet:touch-pan-y",
    ],
    laptop: tw![
        "laptop:max-w-full",
        "laptop:touch-pan-y",
    ],
    desktop: tw![
        "desktop:max-w-full",
        "desktop:touch-pan-y",
    ],
    qhd: tw![
        "qhd:max-w-full",
        "qhd:touch-pan-y",
    ],
    uhd: tw![
        "uhd:max-w-full",
        "uhd:touch-pan-y",
    ],
}
