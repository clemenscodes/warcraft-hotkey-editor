use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-baseline",
        "justify-center",
        "flex-wrap",
        "gap-3",
        "flex-auto",
    ],
    mobile: tw![
        "mobile:hidden",
    ],
    tablet: tw![
        "tablet:hidden",
    ],
    laptop: tw![
        "laptop:hidden",
    ],
    desktop: tw![
        "desktop:hidden",
    ],
    qhd: tw![
        "qhd:gap-4.25",
        "qhd:hidden",
    ],
    uhd: tw![
        "uhd:gap-5",
        "uhd:hidden",
    ],
}
