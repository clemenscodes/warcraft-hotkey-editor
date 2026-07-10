use tw_macro::tw;

// The selected id look: the shared id typography plus the race accent at reduced
// opacity, read from the theme container's `--race-accent` (gold when unthemed).

classes! {
    base: tw![
        "text-base",
        "leading-title",
        "overflow-hidden",
        "text-ellipsis",
        "whitespace-nowrap",
        "opacity-70",
        "text-(--race-accent,var(--color-warcraft-gold))",
    ],
    mobile: tw![
        "mobile:block",
        "mobile:w-full",
        "mobile:text-xs",
        "mobile:leading-title",
    ],
    tablet: tw![
        "tablet:block",
        "tablet:w-full",
        "tablet:text-xs",
        "tablet:leading-title",
    ],
}
