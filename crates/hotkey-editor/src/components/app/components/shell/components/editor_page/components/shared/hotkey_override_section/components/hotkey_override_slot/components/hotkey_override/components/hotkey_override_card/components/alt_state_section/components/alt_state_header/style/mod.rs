use tw_macro::tw;

classes! {
    base: tw![
        "grid",
        "grid-cols-[minmax(0,1fr)_auto_auto]",
        "items-stretch",
        "gap-x-3.5",
        "h-20",
    ],
    mobile: tw![
        "mobile:h-[3.4em]",
        "mobile:gap-x-[0.6em]",
    ],
    tablet: tw![
        "tablet:h-[3.4em]",
        "tablet:gap-x-[0.6em]",
    ],
    laptop: tw![
        "laptop:h-[3.4em]",
        "laptop:gap-x-[0.6em]",
    ],
    desktop: tw![
        "desktop:h-[3.4em]",
        "desktop:gap-x-[0.6em]",
    ],
    qhd: tw![
        "qhd:h-[3.4em]",
        "qhd:gap-x-[0.6em]",
    ],
    uhd: tw![
        "uhd:h-[3.4em]",
        "uhd:gap-x-[0.6em]",
    ],
}
