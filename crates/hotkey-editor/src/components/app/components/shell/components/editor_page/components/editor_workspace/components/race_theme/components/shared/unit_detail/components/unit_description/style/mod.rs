use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "min-h-36",
        "text-xl",
        "leading-body",
        "text-warcraft-text-secondary",
        "text-shadow-drop",
    ],
    // The touch bands render this leaf inside the unit detail dialog, where the
    // whole point is to read the unit trivia. So the description flows to its
    // full height and wraps, rather than clamping to a one line teaser.
    mobile: tw![
        "mobile:flex-none",
        "mobile:min-h-0",
        "mobile:max-w-full",
        "mobile:text-sm",
        "mobile:leading-body",
        "mobile:wrap-break-word",
        "mobile:[word-break:break-word]",
    ],
    tablet: tw![
        "tablet:flex-none",
        "tablet:min-h-0",
        "tablet:max-w-full",
        "tablet:text-sm",
        "tablet:leading-body",
        "tablet:wrap-break-word",
        "tablet:[word-break:break-word]",
    ],
}
