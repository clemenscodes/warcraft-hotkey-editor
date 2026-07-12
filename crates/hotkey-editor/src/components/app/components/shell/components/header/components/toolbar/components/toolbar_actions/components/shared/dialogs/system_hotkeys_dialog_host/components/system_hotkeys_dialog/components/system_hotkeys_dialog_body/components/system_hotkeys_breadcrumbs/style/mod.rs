use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-baseline",
        "justify-center",
        "flex-wrap",
        "gap-3",
        "flex-none",
        "px-8",
        "py-5",
        "[border-bottom:1px_solid_color-mix(in_oklab,var(--color-warcraft-gold)_25%,transparent)]",
    ],
    mobile: tw![
        "mobile:relative",
        "mobile:flex-nowrap",
        "mobile:justify-stretch",
        "mobile:px-3",
        "mobile:py-2",
        "mobile:gap-0",
        "mobile:overflow-visible",
    ],
    tablet: tw![
        "tablet:relative",
        "tablet:flex-nowrap",
        "tablet:justify-stretch",
        "tablet:px-3",
        "tablet:py-2",
        "tablet:gap-0",
        "tablet:overflow-visible",
    ],
}
