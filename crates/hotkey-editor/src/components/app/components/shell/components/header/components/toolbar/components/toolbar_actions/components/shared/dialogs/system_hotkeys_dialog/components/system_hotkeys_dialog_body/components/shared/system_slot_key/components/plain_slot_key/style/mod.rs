use tw_macro::tw;

classes! {
    base: tw![
        "uppercase",
        "tracking-label",
        "text-(length:--slot-key-size,var(--text-4xl))",
        "leading-none",
        "whitespace-nowrap",
        "text-warcraft-gold",
        "text-shadow-glow",
    ],
    mobile: tw![
        "mobile:text-(length:--slot-key-size,var(--text-sm))",
        "mobile:tracking-snug",
    ],
    tablet: tw![
        "tablet:text-(length:--slot-key-size,var(--text-sm))",
        "tablet:tracking-snug",
    ],
    laptop: tw![
        "laptop:text-(length:--slot-key-size,var(--text-sm))",
        "laptop:tracking-snug",
    ],
    desktop: tw![
        "desktop:text-(length:--slot-key-size,var(--text-sm))",
        "desktop:tracking-snug",
    ],
    qhd: tw![
        "qhd:text-(length:--slot-key-size,var(--text-sm))",
        "qhd:tracking-snug",
    ],
    uhd: tw![
        "uhd:text-(length:--slot-key-size,var(--text-sm))",
        "uhd:tracking-snug",
    ],
}
