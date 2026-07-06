use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "gap-[0.45rem]",
        "px-[0.6rem]",
        "py-[0.85rem]",
        "cursor-default",
        "border-2",
        "border-dashed",
        "border-warcraft-gold/18",
        "text-[2.4rem]",
        "text-warcraft-gold/25",
        "[background:color-mix(in_oklab,var(--color-warcraft-bg-base)_50%,transparent)]",
    ],
    mobile: tw![
        "mobile:aspect-[1/0.85]",
        "mobile:text-[1.4rem]",
    ],
    tablet: tw![
        "tablet:aspect-[1/0.85]",
        "tablet:text-[1.4rem]",
    ],
}
