use tw_macro::tw;

// The plain gold key glyph. Its glyph size is inherited from the parent size
// container through `--slot-key-size` (control-group rows tighten it); with no
// container override the fallbacks reproduce the regular hero/inventory sizes.
classes! {
    base: tw![
        "uppercase",
        "tracking-label",
        "text-[length:var(--slot-key-size,var(--text-4xl))]",
        "leading-none",
        "whitespace-nowrap",
        "text-warcraft-gold",
        "text-shadow-glow",
    ],
    mobile: tw![
        "mobile:text-[length:var(--slot-key-size,var(--text-xs))]",
        "mobile:tracking-snug",
    ],
    tablet: tw![
        "tablet:text-[length:var(--slot-key-size,var(--text-xs))]",
        "tablet:tracking-snug",
    ],
}
