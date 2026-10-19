use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "items-center",
        "gap-8",
        "w-full",
    ],
    mobile: tw![
        "mobile:gap-3.5",
        "mobile:px-3",
    ],
    tablet: tw![
        "tablet:gap-3.5",
    ],
    laptop: tw![
        "laptop:gap-3.5",
    ],
    desktop: tw![
        "desktop:gap-3.5",
    ],
    qhd: tw![
        "qhd:gap-5",
    ],
    uhd: tw![
        "uhd:gap-6",
    ],
}
