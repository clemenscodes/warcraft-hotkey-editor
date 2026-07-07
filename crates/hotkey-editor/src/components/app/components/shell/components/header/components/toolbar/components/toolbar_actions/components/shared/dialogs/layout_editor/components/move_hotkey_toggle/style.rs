use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-center",
        "gap-3",
        "uppercase",
        "tracking-[0.06em]",
        "text-2xl",
        "text-warcraft-gold",
        "cursor-pointer",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:gap-2",
        "mobile:text-sm",
    ],
    tablet: tw![
        "tablet:gap-2",
        "tablet:text-sm",
    ],
}
