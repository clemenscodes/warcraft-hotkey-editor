use tw_macro::tw;
classes! {
    base: tw![
        "flex-[1_1_0]",
        "min-h-0",
        "overflow-y-auto",
        "p-[0.75rem_0.75rem_0.75rem_0]",
        "[scrollbar-width:thin]",
        "[scrollbar-color:color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)_transparent]",
    ],
    mobile: tw![
        "mobile:flex-none",
        "mobile:min-h-[auto]",
        "mobile:overflow-y-visible",
        "mobile:p-[0.75rem_0]",
    ],
    tablet: tw![
        "tablet:flex-none",
        "tablet:min-h-[auto]",
        "tablet:overflow-y-visible",
        "tablet:p-[0.75rem_0]",
    ],
}
