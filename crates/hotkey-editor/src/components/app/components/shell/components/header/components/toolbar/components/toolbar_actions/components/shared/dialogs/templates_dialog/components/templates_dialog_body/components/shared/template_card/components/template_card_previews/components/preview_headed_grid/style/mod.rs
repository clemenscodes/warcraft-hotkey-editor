use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "gap-3",
        "pt-2",
        "flex-[1_1_0]",
        "min-w-0",
        "self-stretch",
        "@container",
        "max-w-144.5",
    ],
    mobile: tw![
        "mobile:max-w-120.5",
    ],
    tablet: tw![
        "tablet:max-w-132.5",
    ],
    laptop: tw![
        "laptop:max-w-132.5",
    ],
    desktop: tw![
        "desktop:max-w-132.5",
    ],
    qhd: tw![
        "qhd:gap-4.25",
        "qhd:pt-2.75",
        "qhd:max-w-185.5",
    ],
    uhd: tw![
        "uhd:gap-5",
        "uhd:pt-3.5",
        "uhd:max-w-225.25",
    ],
}
