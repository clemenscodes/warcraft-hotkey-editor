use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-[1_1_0]",
        "justify-center",
        "min-w-0",
    ],
    // Stacked on a phone, each grid is capped to a readable size and centred,
    // so the before and after grids plus the arrow all fit the card height
    // without overflowing. The cap never exceeds the card width on tiny screens.
    mobile: tw![
        "mobile:flex-none",
        "mobile:w-[15em]",
        "mobile:max-w-full",
    ],
    // On a tablet and up the grids sit side by side; each takes 38% of
    // the panel so the arrow between them has room on both sides.
    tablet: tw![
        "tablet:flex-none",
        "tablet:w-[38%]",
    ],
    laptop: tw![
        "laptop:flex-none",
        "laptop:w-[38%]",
    ],
    desktop: tw![
        "desktop:flex-none",
        "desktop:w-[38%]",
    ],
    qhd: tw![
        "qhd:flex-none",
        "qhd:w-[38%]",
    ],
    uhd: tw![
        "uhd:flex-none",
        "uhd:w-[38%]",
    ],
}
