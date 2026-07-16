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
        "bg-warcraft-bg-base/50",
    ],
    mobile: tw![
        "mobile:aspect-[1/0.85]",
        "mobile:text-sm",
    ],
    tablet: tw![
        "tablet:aspect-[1/0.85]",
        "tablet:text-lg",
    ],
}
