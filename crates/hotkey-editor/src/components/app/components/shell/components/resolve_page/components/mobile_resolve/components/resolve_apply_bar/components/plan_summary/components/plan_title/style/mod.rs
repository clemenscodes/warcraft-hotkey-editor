use tw_macro::tw;
classes! {
    base: tw![
        "text-2xl",
        "uppercase",
        "tracking-heading",
        "text-warcraft-gold",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:text-base",
        "mobile:whitespace-nowrap",
    ],
    tablet: tw![
        "tablet:whitespace-nowrap",
        "tablet:text-xl",
    ],
    laptop: tw![
        "laptop:whitespace-nowrap",
        "laptop:text-2xl",
    ],
    desktop: tw![
        "desktop:text-2xl",
        "desktop:whitespace-nowrap",
    ],
    qhd: tw![
        "qhd:text-3xl",
        "qhd:whitespace-nowrap",
    ],
    uhd: tw![
        "uhd:text-4xl",
        "uhd:whitespace-nowrap",
    ],
}
