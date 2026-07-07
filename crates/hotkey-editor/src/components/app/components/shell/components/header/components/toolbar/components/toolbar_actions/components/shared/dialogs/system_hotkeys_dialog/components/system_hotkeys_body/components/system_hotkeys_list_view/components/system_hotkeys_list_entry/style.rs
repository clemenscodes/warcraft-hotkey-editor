use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-center",
        "justify-between",
        "gap-8",
        "px-8",
        "py-5",
        "[border-top:1px_solid_color-mix(in_oklab,var(--color-warcraft-gold)_14%,transparent)]",
        "last:[border-bottom:1px_solid_color-mix(in_oklab,var(--color-warcraft-gold)_14%,transparent)]",
    ],
    mobile: tw![
        "mobile:gap-3",
        "mobile:px-2",
        "mobile:py-3",
        "mobile:[touch-action:pan-y]",
    ],
    tablet: tw![
        "tablet:gap-3",
        "tablet:px-2",
        "tablet:py-3",
        "tablet:[touch-action:pan-y]",
    ],
}
