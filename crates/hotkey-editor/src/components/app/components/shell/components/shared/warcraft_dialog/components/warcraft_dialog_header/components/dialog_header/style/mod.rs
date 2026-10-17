use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "relative",
        "flex",
        "items-center",
        "justify-center",
        "gap-6",
        "flex-none",
        "pt-6",
        "px-18",
        "pb-6",
        "border-b",
        "border-warcraft-gold/40",
    ],
    mobile: tw![
        "mobile:gap-2",
        "mobile:min-h-10",
        "mobile:px-14",
        "mobile:py-3",
    ],
    tablet: tw![
        "tablet:gap-2",
        "tablet:px-14",
    ],
}
