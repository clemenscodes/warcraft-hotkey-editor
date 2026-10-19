use tw_macro::tw;
classes! {
    base: tw![
        "text-xl",
        "text-warcraft-gold",
        "text-shadow-drop",
        "whitespace-nowrap",
        "flex-none",
    ],
    mobile: tw![
        "mobile:text-[0.9em]",
    ],
    tablet: tw![
        "tablet:text-[0.9em]",
    ],
    laptop: tw![
        "laptop:text-[0.9em]",
    ],
    desktop: tw![
        "desktop:text-(length:--collision-count-size,var(--text-xl))",
    ],
    qhd: tw![
        "qhd:text-(length:--collision-count-size,var(--text-xl))",
    ],
    uhd: tw![
        "uhd:text-(length:--collision-count-size,var(--text-xl))",
    ],
}
