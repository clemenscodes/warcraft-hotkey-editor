use tw_macro::tw;

classes! {
    base: tw![
        "text-base",
        "leading-title",
        "text-warcraft-text-faint",
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
