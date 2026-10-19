use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-center",
        "justify-between",
        "gap-8",
        "px-8",
        "py-5",
        "border-t",
        "border-warcraft-gold/14",
        "last:border-b",
        "last:border-warcraft-gold/14",
    ],
    mobile: tw![
        "mobile:gap-3",
        "mobile:px-2",
        "mobile:py-3",
        "mobile:touch-pan-y",
    ],
    tablet: tw![
        "tablet:gap-3",
        "tablet:px-2",
        "tablet:py-3",
        "tablet:touch-pan-y",
    ],
    laptop: tw![
        "laptop:gap-3",
        "laptop:px-2",
        "laptop:py-3",
        "laptop:touch-pan-y",
    ],
    desktop: tw![
        "desktop:gap-3",
        "desktop:px-2",
        "desktop:py-3",
        "desktop:touch-pan-y",
    ],
    qhd: tw![
        "qhd:gap-4.25",
        "qhd:px-2.75",
        "qhd:py-4.25",
        "qhd:touch-pan-y",
    ],
    uhd: tw![
        "uhd:gap-5",
        "uhd:px-3.5",
        "uhd:py-5",
        "uhd:touch-pan-y",
    ],
}
