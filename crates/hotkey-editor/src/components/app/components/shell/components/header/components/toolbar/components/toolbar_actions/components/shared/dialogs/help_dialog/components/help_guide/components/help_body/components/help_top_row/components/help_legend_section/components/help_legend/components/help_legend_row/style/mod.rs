use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-center",
        "gap-5",
    ],
    mobile: tw![
        "mobile:gap-[0.8em]",
    ],
    tablet: tw![
        "tablet:gap-[0.8em]",
        "tablet:whitespace-nowrap",
    ],
    laptop: tw![
        "laptop:gap-[0.8em]",
        "laptop:whitespace-nowrap",
    ],
    desktop: tw![
        "desktop:gap-[0.8em]",
        "desktop:whitespace-nowrap",
    ],
    qhd: tw![
        "qhd:gap-[0.8em]",
        "qhd:whitespace-nowrap",
    ],
    uhd: tw![
        "uhd:gap-[0.8em]",
        "uhd:whitespace-nowrap",
    ],
}
