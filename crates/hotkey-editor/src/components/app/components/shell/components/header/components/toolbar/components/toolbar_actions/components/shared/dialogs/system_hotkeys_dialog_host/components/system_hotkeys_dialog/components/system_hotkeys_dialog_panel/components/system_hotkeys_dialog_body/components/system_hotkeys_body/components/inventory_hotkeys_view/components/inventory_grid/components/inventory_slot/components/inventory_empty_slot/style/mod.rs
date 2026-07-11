use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "gap-2",
        "px-2.5",
        "py-3.5",
        "cursor-default",
        "border-2",
        "border-dashed",
        "border-warcraft-gold/18",
        "text-3xl",
        "text-warcraft-gold/25",
        "[background:color-mix(in_oklab,var(--color-warcraft-bg-base)_50%,transparent)]",
    ],
    mobile: tw![
        "mobile:aspect-[1/0.85]",
        "mobile:text-lg",
    ],
    tablet: tw![
        "tablet:aspect-[1/0.85]",
        "tablet:text-lg",
    ],
}
