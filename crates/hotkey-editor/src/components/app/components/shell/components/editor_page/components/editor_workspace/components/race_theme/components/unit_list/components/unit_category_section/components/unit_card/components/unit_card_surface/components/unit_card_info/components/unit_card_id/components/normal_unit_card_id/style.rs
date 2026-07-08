use tw_macro::tw;

// The muted id look: the shared id typography plus the faint resting text color.

classes! {
    base: tw![
        "text-base",
        "leading-title",
        "overflow-hidden",
        "text-ellipsis",
        "whitespace-nowrap",
        "text-warcraft-text-faint",
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
