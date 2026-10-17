use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-center",
        "gap-3",
        "uppercase",
        "tracking-caps",
        "text-2xl",
        "text-warcraft-gold",
        "cursor-pointer",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:gap-2",
        "mobile:normal-case",
        "mobile:text-sm",
        "mobile:tracking-normal",
    ],
    tablet: tw![
        "tablet:gap-2",
        "tablet:text-sm",
    ],
}
