use tw_macro::tw;
classes! {
    base: tw![
        "text-base",
        "leading-title",
        "pb-0.5",
        "min-w-0",
        "wrap-break-word",
    ],
    mobile: tw![
        "mobile:pb-0",
        "mobile:text-sm",
        "mobile:leading-heading",
    ],
    tablet: tw![
        "tablet:pb-0",
        "tablet:text-base",
        "tablet:leading-heading",
    ],
    laptop: tw![
        "laptop:pb-0",
        "laptop:text-md",
        "laptop:leading-heading",
    ],
    desktop: tw![
        "desktop:text-lg",
    ],
    qhd: tw![
        "qhd:text-lg",
    ],
    uhd: tw![
        "uhd:text-lg",
    ],
}
