use tw_macro::tw;
classes! {
    base: tw![
        "text-3xl",
        "leading-none",
        "uppercase",
        "tracking-heading",
        "text-warcraft-gold",
        "text-glow",
    ],
    // The header is the @container. The font is a fraction of its inline size,
    // so it rides the dialog width the way the header brand does, and the
    // fraction is tuned so the longest title still fits on one line at the
    // narrowest phone, next to the flanking flourishes and the close button.
    // No clip, no ellipsis, no wrap: overflow and truncation are impossible by
    // construction rather than hidden after the fact.
    mobile: tw![
        "mobile:text-[3.5cqi]",
        "mobile:whitespace-nowrap",
    ],
    tablet: tw![
        "tablet:text-[3.5cqi]",
        "tablet:whitespace-nowrap",
    ],
}
