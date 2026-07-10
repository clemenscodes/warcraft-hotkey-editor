use tw_macro::tw;
// The name reads the card's `--name-color` (white on a selected mobile/tablet tile,
// inheriting the surface text otherwise) — no `data-[selected]` selector.
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
        "mobile:block",
        "mobile:w-full",
        "mobile:text-xs",
        "mobile:leading-title",
        "mobile:text-(--name-color,inherit)",
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
