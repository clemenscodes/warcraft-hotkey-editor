use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-1",
        "py-3",
        "text-warcraft-text-secondary",
        "text-lg",
        "leading-prose",
    ],
    mobile: tw![
        "mobile:gap-[0.4em]",
        "mobile:py-[0.5em]",
        "mobile:text-[1em]",
    ],
    tablet: tw![
        "tablet:gap-[0.4em]",
        "tablet:py-[0.5em]",
        "tablet:text-[1em]",
    ],
    laptop: tw![
        "laptop:gap-[0.4em]",
        "laptop:py-[0.5em]",
        "laptop:text-[1em]",
    ],
    desktop: tw![
        "desktop:gap-[0.4em]",
        "desktop:py-[0.5em]",
        "desktop:text-[1em]",
    ],
    qhd: tw![
        "qhd:gap-[0.4em]",
        "qhd:py-[0.5em]",
        "qhd:text-[1em]",
    ],
    uhd: tw![
        "uhd:gap-[0.4em]",
        "uhd:py-[0.5em]",
        "uhd:text-[1em]",
    ],
}
