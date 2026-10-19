use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "items-start",
        "justify-center",
        "gap-6",
        "w-full",
    ],
    mobile: tw![
        "mobile:gap-[1.5em]",
    ],
    tablet: tw![
        "tablet:gap-[1.5em]",
    ],
    laptop: tw![
        "laptop:gap-[1.5em]",
    ],
    desktop: tw![
        "desktop:gap-[1.5em]",
    ],
    qhd: tw![
        "qhd:gap-[1.5em]",
    ],
    uhd: tw![
        "uhd:gap-[1.5em]",
    ],
}
