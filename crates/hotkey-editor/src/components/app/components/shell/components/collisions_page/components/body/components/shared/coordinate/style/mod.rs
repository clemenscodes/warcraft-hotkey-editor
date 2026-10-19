use tw_macro::tw;
classes! {
    base: tw![
        "text-xl",
        "leading-flush",
        "text-(--coordinate-color,var(--color-warcraft-text-primary))",
        "whitespace-nowrap",
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
        "desktop:text-(length:--coordinate-size,var(--text-xl))",
    ],
    qhd: tw![
        "qhd:text-(length:--coordinate-size,var(--text-xl))",
    ],
    uhd: tw![
        "uhd:text-(length:--coordinate-size,var(--text-xl))",
    ],
}
