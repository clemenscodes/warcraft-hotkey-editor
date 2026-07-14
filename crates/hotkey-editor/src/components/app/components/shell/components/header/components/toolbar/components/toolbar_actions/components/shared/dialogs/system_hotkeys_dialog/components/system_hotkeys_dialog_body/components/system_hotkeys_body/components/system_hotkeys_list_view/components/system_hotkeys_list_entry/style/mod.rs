use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-center",
        "justify-between",
        "gap-8",
        "px-8",
        "py-5",
        "border-t",
        "border-warcraft-gold/14",
        "last:border-b",
        "last:border-warcraft-gold/14",
    ],
    mobile: tw![
        "mobile:gap-3",
        "mobile:px-2",
        "mobile:py-3",
        "mobile:touch-pan-y",
    ],
    tablet: tw![
        "tablet:gap-3",
        "tablet:px-2",
        "tablet:py-3",
        "tablet:touch-pan-y",
    ],
}
