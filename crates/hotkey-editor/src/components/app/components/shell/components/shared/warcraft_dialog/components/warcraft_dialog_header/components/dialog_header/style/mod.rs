use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "relative",
        "flex",
        "items-center",
        "justify-center",
        "gap-6",
        "flex-none",
        "pt-6",
        "px-18",
        "pb-6",
        "border-b",
        "border-warcraft-gold/40",
    ],
    mobile: tw![
        "mobile:gap-2",
        "mobile:min-h-10",
        "mobile:px-14",
        "mobile:py-3",
    ],
    tablet: tw![
        "tablet:gap-2",
        "tablet:min-h-10",
        "tablet:px-14",
        "tablet:py-3",
    ],
    laptop: tw![
        "laptop:gap-2",
        "laptop:min-h-10",
        "laptop:px-14",
        "laptop:py-3",
    ],
    desktop: tw![
        "desktop:gap-2",
        "desktop:min-h-10",
        "desktop:px-14",
        "desktop:py-3",
    ],
    qhd: tw![
        "qhd:gap-2.75",
        "qhd:min-h-14",
        "qhd:px-19.5",
        "qhd:py-4.25",
    ],
    uhd: tw![
        "uhd:gap-3.5",
        "uhd:min-h-17",
        "uhd:px-23.75",
        "uhd:py-5",
    ],
}
