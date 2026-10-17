use tw_macro::tw;
classes! {
    base: tw![
        "text-base",
        "leading-title",
        "pb-0.5",
        "overflow-hidden",
        "text-ellipsis",
        "whitespace-nowrap",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:pb-0",
        "mobile:text-sm",
        "mobile:leading-heading",
    ],
    tablet: tw![
        "tablet:block",
        "tablet:w-full",
        "tablet:text-xs",
        "tablet:leading-title",
        "tablet:text-(--name-color,inherit)",
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
