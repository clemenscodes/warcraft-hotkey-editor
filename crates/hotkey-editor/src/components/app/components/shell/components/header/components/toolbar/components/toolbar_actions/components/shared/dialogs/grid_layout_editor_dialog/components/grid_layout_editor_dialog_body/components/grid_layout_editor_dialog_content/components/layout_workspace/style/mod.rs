use tw_macro::tw;
// Holds the grid and its toggle together. On a phone or tablet it takes the room
// between the intro and the apply button and centres the pair in it.
classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "gap-16",
    ],
    mobile: tw![
        "mobile:flex-1",
        "mobile:justify-center-safe",
        "mobile:w-full",
        "mobile:gap-5",
    ],
    tablet: tw![
        "tablet:justify-start",
        "tablet:w-[40dvh]",
        "tablet:gap-5",
    ],
    laptop: tw![
        "laptop:justify-start",
        "laptop:w-[40dvh]",
        "laptop:gap-5",
    ],
    desktop: tw![
        "desktop:justify-start",
        "desktop:w-[40dvh]",
        "desktop:gap-5",
    ],
    qhd: tw![
        "qhd:justify-start",
        "qhd:w-[40dvh]",
        "qhd:gap-7",
    ],
    uhd: tw![
        "uhd:justify-start",
        "uhd:w-[40dvh]",
        "uhd:gap-8.5",
    ],
}
