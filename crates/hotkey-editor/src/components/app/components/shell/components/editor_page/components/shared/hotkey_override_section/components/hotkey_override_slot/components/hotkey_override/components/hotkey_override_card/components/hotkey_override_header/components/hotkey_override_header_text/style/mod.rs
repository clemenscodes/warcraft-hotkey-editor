use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-start",
        "gap-1.5",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:gap-[0.15em]",
        "mobile:min-h-0",
        "mobile:justify-center",
        "mobile:text-left",
    ],
    tablet: tw![
        "tablet:gap-[0.15em]",
        "tablet:min-h-0",
        "tablet:justify-center",
        "tablet:text-left",
    ],
    laptop: tw![
        "laptop:gap-[0.15em]",
        "laptop:min-h-0",
        "laptop:justify-center",
        "laptop:text-left",
    ],
    desktop: tw![
        "desktop:gap-[0.15em]",
        "desktop:min-h-0",
        "desktop:justify-center",
        "desktop:text-left",
    ],
    qhd: tw![
        "qhd:gap-[0.15em]",
        "qhd:min-h-0",
        "qhd:justify-center",
        "qhd:text-left",
    ],
    uhd: tw![
        "uhd:gap-[0.15em]",
        "uhd:min-h-0",
        "uhd:justify-center",
        "uhd:text-left",
    ],
}
