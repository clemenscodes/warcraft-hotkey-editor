use tw_macro::tw;
classes! {
    base: tw![
        "text-xl",
        "text-warcraft-text-primary",
        "whitespace-normal",
        "leading-flush",
    ],
    mobile: tw![
        "mobile:text-[1.35em]",
    ],
    tablet: tw![
        "tablet:text-[1.35em]",
    ],
    laptop: tw![
        "laptop:text-[1.35em]",
    ],
    desktop: tw![
        "desktop:text-(length:--unit-name-size,var(--text-xl))",
    ],
    qhd: tw![
        "qhd:text-(length:--unit-name-size,var(--text-xl))",
    ],
    uhd: tw![
        "uhd:text-(length:--unit-name-size,var(--text-xl))",
    ],
}
