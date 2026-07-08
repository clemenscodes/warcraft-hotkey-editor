use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-5",
        "flex-[1_1_0]",
        "min-h-0",
        "overflow-y-auto",
        "pt-3", "pr-3", "pb-3", "pl-0",
        "scrollbar-thin",
        "[scrollbar-color:color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)_transparent]",
    ],
    mobile: tw![
        "mobile:flex-none",
        "mobile:min-h-auto",
        "mobile:overflow-y-visible",
        "mobile:py-3", "mobile:px-0",
    ],
    tablet: tw![
        "tablet:flex-none",
        "tablet:min-h-auto",
        "tablet:overflow-y-visible",
        "tablet:py-3", "tablet:px-0",
    ],
}
