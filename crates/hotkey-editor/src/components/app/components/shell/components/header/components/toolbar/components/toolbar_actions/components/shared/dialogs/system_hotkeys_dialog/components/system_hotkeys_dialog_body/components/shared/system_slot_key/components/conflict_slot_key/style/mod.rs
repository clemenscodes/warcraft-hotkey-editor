use tw_macro::tw;

classes! {
    base: tw![
        "uppercase",
        "tracking-label",
        "text-(length:--slot-key-size,var(--text-4xl))",
        "leading-none",
        "whitespace-nowrap",
        "text-warcraft-danger",
        "[--glow-color:var(--color-warcraft-danger)]",
        "text-glow",
    ],
    mobile: tw![
        "mobile:text-(length:--slot-key-size,var(--text-xs))",
        "mobile:tracking-snug",
    ],
    tablet: tw![
        "tablet:text-(length:--slot-key-size,var(--text-xs))",
        "tablet:tracking-snug",
    ],
    laptop: tw![
        "laptop:text-(length:--slot-key-size,var(--text-xs))",
        "laptop:tracking-snug",
    ],
    desktop: tw![
        "desktop:text-(length:--slot-key-size,var(--text-xs))",
        "desktop:tracking-snug",
    ],
    qhd: tw![
        "qhd:text-(length:--slot-key-size,var(--text-xs))",
        "qhd:tracking-snug",
    ],
    uhd: tw![
        "uhd:text-(length:--slot-key-size,var(--text-xs))",
        "uhd:tracking-snug",
    ],
}
