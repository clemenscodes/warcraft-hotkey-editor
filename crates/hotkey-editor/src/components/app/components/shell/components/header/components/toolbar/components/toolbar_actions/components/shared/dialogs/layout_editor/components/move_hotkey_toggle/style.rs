use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-center",
        "gap-[0.8rem]",
        "uppercase",
        "tracking-[0.06em]",
        "text-[1.9rem]",
        "text-warcraft-gold",
        "cursor-pointer",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:gap-[8px]",
        "mobile:text-[15px]",
    ],
    tablet: tw![
        "tablet:gap-[8px]",
        "tablet:text-[15px]",
    ],
}
