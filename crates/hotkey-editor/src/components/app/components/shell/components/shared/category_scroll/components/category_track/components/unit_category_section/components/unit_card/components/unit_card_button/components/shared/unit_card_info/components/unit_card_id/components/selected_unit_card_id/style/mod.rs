use tw_macro::tw;

classes! {
    base: tw![
        "text-base",
        "leading-title",
        "opacity-70",
        "text-(--race-color,var(--color-warcraft-gold))",
        "wrap-break-word",
    ],
    mobile: tw![
        "mobile:text-sm",
        "mobile:leading-heading",
    ],
    tablet: tw![
        "tablet:text-base",
        "tablet:leading-heading",
    ],
    laptop: tw![
        "laptop:text-md",
        "laptop:leading-heading",
    ],
}
