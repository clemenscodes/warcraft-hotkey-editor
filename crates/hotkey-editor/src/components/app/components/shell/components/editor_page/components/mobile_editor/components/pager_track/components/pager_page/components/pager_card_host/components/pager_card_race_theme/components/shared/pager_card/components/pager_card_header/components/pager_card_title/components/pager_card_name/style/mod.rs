use tw_macro::tw;
classes! {
    base: tw![
        "block",
        "text-[8cqi]",
        "leading-flush",
        "font-semibold",
        "text-(--race-color,var(--color-warcraft-gold))",
        "text-shadow-drop",
        "wrap-break-word",
    ],
    tablet: tw![
        "tablet:text-[4cqi]",
    ],
    laptop: tw![
        "laptop:text-[4cqi]",
    ],
}
