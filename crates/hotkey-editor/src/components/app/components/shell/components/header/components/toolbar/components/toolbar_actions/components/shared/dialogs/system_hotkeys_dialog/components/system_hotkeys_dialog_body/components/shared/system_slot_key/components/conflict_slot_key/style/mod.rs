use tw_macro::tw;

// The danger-red key glyph shown when the slot's binding collides with another. Same
// structural glyph as `PlainSlotKey` (size inherited from the parent size container
// through `--slot-key-size`), recoloured danger-red with a matching red glow.
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
}
